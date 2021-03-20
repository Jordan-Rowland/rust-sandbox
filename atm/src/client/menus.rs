use super::helpers::{input, num_input, atm_exit};

use crate::account::Account;


pub enum Menu {
    Withdraw,
    Deposit,
    Transfer,
    Exit,
}


pub fn sign_in() -> String {
    // !  This needs more functionality
    input("Please enter an account id")
}


pub fn main_menu(account: &mut Account) {
    loop {
        println!("\n=====");
        // !  display account details
        println!("\nAccount ID: {} - Account Balance: ${}\n\n", account.get_id(), account.get_balance());
        println!("\
        \t(W) - Withdraw money\n\
        \t(D) - Deposit money\n\
        \t(T) - Transfer money\n\
        \t(E) - Exit\n\
        ");
        println!("=====\n");
        let user_action = input("Please select an option");
        // let user_action = "W";
        println!("{}", &user_action[..]);  // ! Delete this
        let action = match &user_action[..] {
            "W" => Menu::Withdraw,
            "D" => Menu::Deposit,
            "T" => Menu::Transfer,
            "E" => Menu::Exit,
            _  => {
                println!("{} is not a valid option.", &user_action.to_uppercase().trim()[..]);
                continue
            }
        };
        action_menu(action, account);
    }
}


pub fn action_menu(action: Menu, account: &mut Account) {
    match action {
        Menu::Withdraw => withdraw_menu(account),
        Menu::Deposit => deposit_menu(account),
        Menu::Transfer => transfer_menu(account),
        Menu::Exit => atm_exit(account),
    }
}


pub fn withdraw_menu(account: &mut Account) {
    loop {
        let action = input("Please select an amount to withdraw, or E to exit");
        if action == "E" {
            break
        }
        let transfer = match action.parse::<u32>() {
            Ok(withdraw_value) => {
                match account.decrease_balance(withdraw_value) {
                    Ok(v) => println!("=====\nAccount Balance: {}\n=====", v),
                    Err(e) => println!("=====\n{}\n=====", e)
                }

            },
            Err(value) => {
                println!("Not a valid action: {}", value);
                continue
            }
        };
        // match transfer {
        //         Ok(_) => {
        //                 println!("{:?}", account);
        //                 break
        //             },
        //             Err(e) => {
        //                     println!("{}", e);
        //                     continue
        //                 },
        //             }
    }
                // println!("Withdraw from {:?}", account);
}


pub fn deposit_menu(account: &Account) {
    println!("Deposit to {:?}", account);
}


pub fn transfer_menu(account: &Account) {
    println!("Transfer from {:?}", account);
}
