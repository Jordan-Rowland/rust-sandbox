#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
use std::io::{self, Write};


use atm::account::Account;
use atm::bank;


fn main() {

    let accounts_raw = "
        id,balance
        1234,2000
        4321,2000
    ";

    let mut accounts: Vec<(String, i64)> = Vec::new();
    let mut lines: Vec<(String, String)> = Vec::new();
    let mut headers: (String, String) = (String::new(), String::new());
    for line in accounts_raw.split('\n') {
        let line = line.trim();
        if line.is_empty() {
            continue
        }
        let mut line_split = line.split(',');
        lines.push(
            (
                String::from(line_split.next().unwrap()),
                String::from(line_split.next().unwrap())
            )
        );
    }

    for (i, line) in lines.into_iter().enumerate() {
        if i == 0 {
            headers = line.to_owned();
        }
        else {
            accounts.push((line.0.to_owned(), line.1.parse::<i64>().unwrap()));
        }
    }

    let mut account1 = Account::from_id("1234", &accounts).unwrap();
    let mut account2 = Account::from_id("4321", &accounts).unwrap();
    bank::transfer_balance(&mut account2, &mut account1, 3500);


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


    // println!("{:?}", std::env::args(0));
    println!("{:?}", std::env::args().nth(1).expect("No args passed"));
}
