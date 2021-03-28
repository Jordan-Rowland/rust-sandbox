use crate::account::Account;
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

pub fn atm_exit(account: &Account) {
    // Clean up here, save to db, etc
    println!("Exiting {:?}", account);
    std::process::exit(0);
}
