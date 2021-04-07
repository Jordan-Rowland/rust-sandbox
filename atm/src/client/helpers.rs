use crate::account::Account;
use crate::data::AccountsData;
use std::io::{self, Write};

pub fn input(prompt: &str) -> String {
    print!("{}\n   > ", prompt);
    io::stdout().flush();
    let mut user_action = String::new();
    io::stdin().read_line(&mut user_action).unwrap();
    user_action.to_uppercase().trim().to_owned()
}

pub fn num_input(prompt: &str) -> Result<u32, std::num::ParseIntError> {
    print!("{}\n   > ", prompt);
    io::stdout().flush();
    let mut user_action = String::new();
    io::stdin().read_line(&mut user_action).unwrap();
    user_action.trim().parse()
}

pub fn atm_exit(account: &Account, accounts: &mut AccountsData) {
    // TODO: Clean up here, save to db, etc
    accounts.write_json_data("./src/data/db.json");
    accounts.write_csv_data("./src/data/db.csv");
    println!("Exiting {:?}", account);
    std::process::exit(0);
}
