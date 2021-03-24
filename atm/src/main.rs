#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use std::collections::HashMap;
use std::io::{self, Write};

// use serde::{Serialize, Deserialize};
use atm::account::Account;
use atm::bank;
use atm::client;
use atm::data;

fn main() {
    let data = data::AccountsData::read_csv_data();
    let mut accounts = Vec::with_capacity(5);
    if let Ok(contents) = data {
        accounts = contents;
    }
    // println!("{:?}", accounts);

    // let mut account1 = Account::from_id("8675309", &accounts).unwrap();
    // let mut account2 = Account::from_id("4815162", &accounts).unwrap();
    // match bank::transfer_balance(&mut account2, &mut account1, 5000) {
    //     Ok(_) => println!("Transaction Successful."),
    //     Err(e) => println!("\nTransaction Failed:\n{}\n", e),
    // }

    // println!("{:?}", account1);
    // println!("{:?}", account2);

    // client::menus::main_menu(&mut account1);

    // print!("Please enter an account id > ");
    // io::stdout().flush();
    // let mut user_supplied_id = String::new();
    // io::stdin().read_line(&mut user_supplied_id).expect("oooops");
    // user_supplied_id = user_supplied_id.trim().to_owned();

    // match Account::from_id(&user_supplied_id, &accounts) {
    //     Some(user_account) => {
    //         println!("Hello user {}, your balance is ${}", user_account.id, user_account.balance);
    //     }
    //     None => println!("No account found with ID {}", user_supplied_id)
    // }

    // let mut row = HashMap::new();
    // row.insert("id".to_string(), data::RowValue::Id("12345".to_string()));
    // // row.insert("balance".to_string(), data::RowValue::Balance(2750));
    // // row.insert("pin".to_string(), data::RowValue::Pin(1029));
    // // row.insert("protected".to_string(), data::RowValue::Protected(true));

    // // let row = data::RowValue::Id("12345".to_string());
    // let ad = data::AccountsData { rows: vec![row] };

    // println!("\n\n\n{:?}", ad);
    // println!("\n\n\n{:?}", s);

    let a = data::AccountsData::read_json_data();



    // println!("{:?}", fmt);
}
