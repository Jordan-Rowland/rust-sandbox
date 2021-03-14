#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use crate::account::Account;
use crate::account::TransactionType;


enum TransactionSecurity {
    Protected,
    Unproteted,
}

impl Account {
    fn increase_balance(&mut self, amount: i64) -> i64 {
        self.set_balance(amount, TransactionType::Add)
    }

    fn decrease_balance(
            &mut self,amount: i64,
            security: TransactionSecurity) -> Result<i64, String> {
        let secured = matches!(security, TransactionSecurity::Protected);
        if secured && self.get_balance() < amount {
            Err(format!(
                "This transaction is protected will not allow \
                a charge(${}) greater than the account balance(&{}). \
                Please run this transaction with TransactionSecurity\
                ::Unprotected to allow an overdraft.",
                amount, self.get_balance()
            ))
        } else {
            Ok(self.set_balance(amount, TransactionType::Subtract))
        }
    }
}

pub fn transfer_balance(
        from_account: &mut Account,
        to_account: &mut Account,
        amount: i64) {
    match from_account.decrease_balance(amount, TransactionSecurity::Protected) {
        Ok(_) => {to_account.increase_balance(amount);},
        Err(e) => println!("Transaction Failed: {}", e)
    }
}

