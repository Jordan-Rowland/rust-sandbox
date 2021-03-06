use super::helpers::{atm_exit, input, num_input};

use crate::data::AccountsData;
use crate::{account::Account, bank};

pub enum Menu {
    Withdraw,
    Deposit,
    Transfer,
    Exit,
}

pub fn main_menu(accounts: &mut AccountsData) {
    let mut account = Account::new("123".to_string(), 123, 123);
    loop {
        let action = input("Please enter an account id, or E to exit");
        if action == "E" {
            println!("Goodbye!");
            return ();
        }
        if let Some(found_account) = Account::from_id(&action, accounts) {
            account = found_account;
            break;
        } else {
            println!("Could not find account ID: {}", action);
            continue;
        }
    }
    loop {
        println!("\n=====");
        println!(
            "\nAccount ID: {} - Account Balance: ${}\n\n",
            account.get_id(),
            account.get_balance()
        );
        println!(
            "\
        \t(W) - Withdraw money\n\
        \t(D) - Deposit money\n\
        \t(T) - Transfer money\n\
        \t(E) - Exit\n\
        "
        );
        println!("=====\n");
        let user_action = input("Please select an option");
        // let user_action = "W";
        // println!("{}", &user_action[..]); // ! Delete this
        let action = match &user_action[..] {
            "W" => Menu::Withdraw,
            "D" => Menu::Deposit,
            "T" => Menu::Transfer,
            "E" => Menu::Exit,
            _ => {
                println!(
                    "{} is not a valid option.",
                    user_action.to_uppercase().trim()
                );
                continue;
            }
        };
        action_menu(action, &mut account, accounts);
    }
}

pub fn action_menu(action: Menu, account: &mut Account, accounts: &mut AccountsData) {
    match action {
        Menu::Withdraw => withdraw_menu(account, accounts),
        Menu::Deposit => deposit_menu(account, accounts),
        Menu::Transfer => transfer_menu(account, accounts),
        Menu::Exit => atm_exit(account, accounts),
    }
}

pub fn withdraw_menu(account: &mut Account, accounts: &mut AccountsData) {
    loop {
        let action = input("Please select an amount to withdraw, or E to exit");
        if action == "E" {
            break;
        }
        let transfer = match action.parse::<u32>() {
            Ok(withdraw_value) => match account.decrease_balance(withdraw_value) {
                Ok(value) => {
                    accounts.update(account.to_row());
                    println!("=====\nAccount Balance: {}\n=====", value);
                },
                Err(e) => println!("=====\n{}\n=====", e),
            },
            Err(value) => {
                println!("Not a valid action: {}", value);
                continue;
            }
        };
        // TODO:
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
}

pub fn deposit_menu(account: &mut Account, accounts: &mut AccountsData) {
    loop {
        let action = input("Please select an amount to deposit`, or E to exit");
        if action == "E" {
            break;
        }
        let deposit = match action.parse::<u32>() {
            Ok(withdraw_value) => {
                let new_balance = account.increase_balance(withdraw_value);
                accounts.update(account.to_row());
                println!("=====\nAccount Balance: {}\n=====", new_balance);
            }
            Err(value) => {
                println!("Not a valid action: {}", value);
                continue;
            }
        };
    }
}

pub fn transfer_menu(account: &mut Account, accounts: &mut AccountsData) {
    loop {
        let action_account = input("Please enter an account id, or E to exit");
        if action_account == "E" {
            break;
        }
        let action_amount = input("Please enter an amount to transfer, or E to exit");
        if action_amount == "E" {
            break;
        }

        match (
            Account::from_id(&action_account, accounts),
            action_amount.parse::<u32>(),
        ) {
            (None, _) => {
                println!("{} is not a valid account ID.", action_amount);
                continue;
            }
            (_, Err(_)) => {
                println!("{} is not a valid amount.", action_amount);
                continue;
            }
            (Some(mut found_account), Ok(parsed_amount)) => {
                match bank::transfer_balance(account, &mut found_account, parsed_amount) {
                    Ok(amount) => {
                        println!(
                            "Successfully transferred ${} to account {}.",
                            parsed_amount,
                            found_account.get_id()
                        );
                        accounts.update(account.to_row());
                        accounts.update(found_account.to_row());
                        break;
                    }
                    Err(e) => {
                        println!("{}", e);
                        break;
                    }
                }
            }
        }
    }
}
