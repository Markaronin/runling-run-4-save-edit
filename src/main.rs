use std::{
    fs::{self, read_to_string, write},
    io::{BufRead, Write},
    path::Path,
};

use bank::{Bank, XmlBank};
use clap::{Parser, Subcommand, ValueEnum};
use model::Runling;

mod bank;
mod model;
mod starcode;

#[derive(Debug, Parser)]
#[command(
    about = "This is a command line utility to help with editing Starcraft 2 bank files for the arcade game 'Runling Run 4'. Prior to using this program, you should have already run Starcraft 2 once and played a game of Runling Run 4, in order to populate the initial bank file. "
)]
struct Args {
    #[arg(
        short,
        long,
        help = "The location of the SC2 Runling Run bank file that you want to edit. This is usually located (on windows) at ~/Documents/Starcraft II/Accounts/<ACCOUNT_ID>/1-S2-1-<USER_ID>/Banks/1-S2-1-417073/RunlingRun004.SC2Bank"
    )]
    file_location: String,
    #[arg(
        short,
        long,
        help = "The location to store an unmodified backup of your bank file"
    )]
    backup_location: String,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Whether to overwrite the backup file if it already exists"
    )]
    overwrite_backup: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum RunlingType {
    Zergling,
    Baneling,
    Hydralisk,
    Ultralisk,
    Roach,
}
impl RunlingType {
    pub fn to_bank_data(&self) -> usize {
        match self {
            RunlingType::Zergling => 1,
            RunlingType::Baneling => 2,
            RunlingType::Hydralisk => 3,
            RunlingType::Ultralisk => 4,
            RunlingType::Roach => 5,
        }
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(
        about = "Unlocks the two score-gated playable units by setting your total score to 50,000."
    )]
    UnlockAllUnits,
    #[command(about = "Creates a new save game with the given attributes")]
    CreateNewUnit {
        #[arg(
            short,
            long,
            help = "The type of the runling you want to create - more info can be found on the arcade game character selection screen"
        )]
        runling_type: RunlingType,
        #[arg(short, long)]
        level: usize,
    },
}

fn get_user_input(prompt: &str) -> String {
    print!("{prompt}");
    std::io::stdout().flush().unwrap();
    std::io::stdin().lock().lines().next().unwrap().unwrap()
}

fn save_bank(bank: Bank, save_location: &str) {
    println!("Saving edited bank data");
    let xml_bank = XmlBank::from(bank);
    let raw_bank = xml_bank.to_correctly_formatted_bank();
    write(save_location, raw_bank).expect("Failed to write edited bank data");
}

fn main() {
    let args = Args::parse();
    let file_location = Path::new(&args.file_location);
    let backup_location = Path::new(&args.backup_location);
    assert!(
        file_location.exists(),
        "Passed bank file location does not exist"
    );
    if !args.overwrite_backup {
        assert!(
            !backup_location.exists(),
            "There already exists a file at the backup location - if you want to overwrite it, use the --overwrite-backup flag"
        );
    }

    fs::copy(&args.file_location, &args.backup_location)
        .expect("Failed to copy original bank to backup file location");
    println!("Successfully created backup file");

    let raw_bank: String = read_to_string(&args.file_location).unwrap();
    let xml_bank: XmlBank = quick_xml::de::from_str(&raw_bank).unwrap();
    let mut bank = Bank::from(xml_bank);

    println!("Successfully read and parsed bank file");

    match args.command {
        Command::UnlockAllUnits => {
            if get_user_input(&format!(
                "Are you sure you want to overwrite your current score of {} to 50,000? y/n ",
                bank.account.total_score
            )) == "y"
            {
                bank.account.total_score = 50000;
                save_bank(bank, &args.file_location)
            } else {
                println!("Action cancelled, exiting")
            }
        }
        Command::CreateNewUnit {
            runling_type,
            level,
        } => {
            let new_unit = Runling {
                class: runling_type.to_bank_data(),
                experience: Runling::experience_from_level(level),
                energy_regeneration: 0,
                maximum_energy: 0,
                speed: 0,
                skill_1_level: 0,
                skill_2_level: 0,
                runling_level: level,
                remaining_points: level * 4,
            };

            let empty_save_slots = (0..=7_usize)
                .filter(|i| bank.units[*i] == None)
                .collect::<Vec<_>>();

            let new_unit_slot = get_user_input(&format!("Which save slot would you like to overwrite with the new unit? Currently slots {empty_save_slots:?} are empty. If you choose to overwrite a full save slot, a confirmation prompt with the unit information will be shown first. \n> "));
            let new_unit_slot = new_unit_slot
                .parse::<usize>()
                .expect("Must input a valid positive number");
            // Don't need to check whether it's greater than zero because it's an unsigned integer
            assert!(
                new_unit_slot <= 7,
                "Must input a number between 0 and 7, inclusive."
            );
            if !empty_save_slots.contains(&new_unit_slot) {
                println!("The chosen slot currently has a runling with the following info in it:");
                println!("{:#?}", bank.units[new_unit_slot]);
                if get_user_input("Are you sure you want to overwrite it? y/n ") != "y" {
                    panic!("Aborting");
                }
            }

            bank.units[new_unit_slot] = Some(new_unit);

            save_bank(bank, &args.file_location);
        }
    }
}
