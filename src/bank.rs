use num::BigInt;
use quick_xml::se::Serializer;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

use crate::{
    model::{Account, Runling},
    starcode::{self, get_int, store_int},
};

fn derive_player_handle(checksum: String, runlings: &[Option<Runling>; 8]) -> usize {
    let mut data = starcode::uncompress(starcode::decrypt(checksum));
    let runling_checksums = get_int(&mut data, 98000000);
    let handle = runling_checksums
        - runlings
            .iter()
            .flatten()
            .map(|r| r.checksum())
            .sum::<usize>();
    handle
}

fn compute_signature(player_handle: usize, mut sections: Vec<Section>) -> String {
    const HANDLE_PREFIX_NA: &str = "1-S2-1-";
    const AUTHOR_HANDLE_NA: usize = 417073;
    const BANK_FILENAME: &str = "RunlingRun004";

    let mut hasher = Sha1::new();

    hasher.update(HANDLE_PREFIX_NA);
    hasher.update(&AUTHOR_HANDLE_NA.to_string());
    hasher.update(HANDLE_PREFIX_NA);
    hasher.update(&player_handle.to_string());
    hasher.update(BANK_FILENAME);

    sections.sort_by(|a, b| a.name.cmp(&b.name));

    for mut section in sections {
        hasher.update(section.name);
        section.keys.sort_by(|a, b| a.name.cmp(&b.name));
        dbg!(&section.keys);
        for key in section.keys {
            hasher.update(key.name);
            hasher.update("Value");
            hasher.update("string");
            hasher.update(key.value.string);
        }
    }

    let hash = hasher.finalize();
    base16ct::upper::encode_string(&hash)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Value {
    #[serde(rename = "@string")]
    string: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Key {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "Value")]
    value: Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Section {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "Key")]
    keys: Vec<Key>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Signature {
    #[serde(rename = "@value")]
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "Bank")]
pub struct XmlBank {
    #[serde(rename = "@version")]
    version: usize,

    #[serde(rename = "Section")]
    section: (Section, Section),

    #[serde(rename = "Signature")]
    signature: Signature,
}
impl XmlBank {
    pub fn to_correctly_formatted_bank(self) -> String {
        let prefix_string = r#"<?xml version="1.0" encoding="utf-8"?>"#.to_string() + "\n";

        let mut buffer = String::new();
        let mut ser = Serializer::new(&mut buffer);
        ser.indent(' ', 4);

        self.serialize(ser).unwrap();

        prefix_string + &buffer + "\n"
    }
}
impl From<Bank> for XmlBank {
    fn from(value: Bank) -> Self {
        let mut unit_section: Section = Section {
            name: "unit".to_string(),
            keys: Vec::new(),
        };
        for unit_index in 1..=8 {
            if let Some(unit) = &value.units[unit_index - 1] {
                let unit_string = unit.to_data();
                unit_section.keys.push(Key {
                    name: format!("0{unit_index}"),
                    value: Value {
                        string: unit_string,
                    },
                });
            }
        }
        unit_section.keys.push(Key {
            name: format!("info"),
            value: Value {
                string: value.generate_unit_slots_data(),
            },
        });

        let account_info = value.account.to_data();
        let account_camera = value.generate_camera_checksum();
        let account_section = Section {
            name: "account".to_string(),
            keys: vec![
                Key {
                    name: "camera".to_string(),
                    value: Value {
                        string: account_camera,
                    },
                },
                Key {
                    name: "info".to_string(),
                    value: Value {
                        string: account_info,
                    },
                },
            ],
        };

        let signature = Signature {
            value: compute_signature(
                value.player_handle,
                vec![unit_section.clone(), account_section.clone()],
            ),
        };

        XmlBank {
            version: 1,
            section: (unit_section, account_section),
            signature,
        }
    }
}

#[derive(Debug)]
pub struct Bank {
    pub units: [Option<Runling>; 8],
    pub account: Account,
    pub player_handle: usize,
}
impl Bank {
    pub fn generate_camera_checksum(&self) -> String {
        let mut data = BigInt::ZERO;
        store_int(&mut data, self.account.checksum(), 99000000);
        store_int(
            &mut data,
            self.units
                .iter()
                .flatten()
                .map(|r| r.checksum())
                .sum::<usize>()
                + self.player_handle,
            98000000,
        );
        starcode::encrypt(starcode::compress(data))
    }

    pub fn generate_unit_slots_data(&self) -> String {
        let mut data = BigInt::ZERO;
        for unit_index in 0..8 {
            let val = if self.units[unit_index].is_some() {
                1
            } else {
                0
            };
            let max_val = 425 + unit_index;
            store_int(&mut data, val, max_val);
        }
        starcode::encrypt(starcode::compress(data))
    }
}
impl From<XmlBank> for Bank {
    fn from(value: XmlBank) -> Self {
        let (unit_section, account_section) = if value.section.0.name == "unit" {
            value.section
        } else {
            (value.section.1, value.section.0)
        };
        assert_eq!(unit_section.name, "unit");
        assert_eq!(account_section.name, "account");

        let checksum = account_section
            .keys
            .iter()
            .find(|key| key.name == "camera")
            .unwrap()
            .value
            .string
            .clone();

        let account = Account::from_data(
            account_section
                .keys
                .iter()
                .find(|key| key.name == "info")
                .unwrap()
                .value
                .string
                .clone(),
        );

        let mut units: [Option<Runling>; 8] = [None, None, None, None, None, None, None, None];
        for runling_index in 1..=8 {
            let runling_info = unit_section
                .keys
                .iter()
                .find(|key| key.name == format!("0{runling_index}"));

            if let Some(runling_info) = runling_info {
                units[runling_index - 1] =
                    Some(Runling::from_data(runling_info.value.string.clone()));
            }
        }

        let player_handle = derive_player_handle(checksum, &units);

        Self {
            units,
            account,
            player_handle,
        }
    }
}
