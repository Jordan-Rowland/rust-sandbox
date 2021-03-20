#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

// use std::collections::HashMap;
use std::io::{self, Write};

// use serde::{Serialize, Deserialize};

use atm::account::Account;
use atm::bank;
use atm::client;
use atm::data;


fn main() {

    // let accounts_raw = "
    //     id,balance,pin,protected
    //     8675309,2000,1234,true
    //     4815162,2000,4321,true
    // ";

    // let mut accounts: Vec<(String, i64, u16, bool)> = Vec::new();
    // let mut lines: Vec<(String, String, String, String)> = Vec::new();
    // let mut headers: (String, String, String, String) = (
    //     String::new(),
    //     String::new(),
    //     String::new(),
    //     String::new(),
    // );
    // for line in accounts_raw.split('\n') {
    //     let line = line.trim();
    //     if line.is_empty() {
    //         continue
    //     }
    //     let mut line_split = line.split(',');
    //     #[cfg(debug_assertions)]  // ! print on debug only
    //     println!("{}", line);  // ! print on debug only
    //     lines.push(
    //         (
    //             line_split.next().unwrap().to_owned(),
    //             line_split.next().unwrap().to_owned(),
    //             line_split.next().unwrap().to_owned(),
    //             line_split.next().unwrap().to_owned(),
    //         )
    //     );
    // }

    // for (i, line) in lines.into_iter().enumerate() {
    //     if i == 0 {
    //         headers = line.to_owned();
    //     } else {
    //         #[cfg(debug_assertions)]  // ! print on debug only
    //         println!("{:?}", line);  // ! print on debug only
    //         accounts.push((
    //             line.0.to_owned(),
    //             line.1.parse().unwrap(),
    //             line.2.parse().unwrap(),
    //             line.3.parse().unwrap(),
    //         ));
    //     }
    // }

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
    // row.insert(
    //     "id".to_string(),
    //     data::RowValue::Id("12345".to_string()))
    // ;

    let row = data::RowValue::Id("12345".to_string());
    let ad = data::AccountData { rows: vec![row] };

    println!("\n\n\n{:?}", ad);
    // let s = serde_json::to_string(&ad).unwrap();
    // println!("\n\n\n{:?}", s);
}
