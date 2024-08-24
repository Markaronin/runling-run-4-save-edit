use std::fs::{read_to_string, write};

use bank::{Bank, XmlBank};

mod bank;
mod model;
mod starcode;

fn main() {
    let raw_bank: String = read_to_string("./data/RunlingRun004.SC2Bank").unwrap();
    let xml_bank: XmlBank = quick_xml::de::from_str(&raw_bank).unwrap();
    dbg!(&xml_bank);
    let mut bank = Bank::from(xml_bank);

    // TODO:

    dbg!(&bank);

    let xml_bank = XmlBank::from(bank);
    let raw_bank = xml_bank.to_correctly_formatted_bank();
    write("./out.xml", raw_bank).unwrap();
}
