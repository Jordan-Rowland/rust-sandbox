#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use crate::account::Account;
use crate::account::TransactionType;


impl Account {
    fn increase_balance(&mut self, amount: u32) -> i64 {
        self.set_balance(amount, TransactionType::Add)
    }

    pub fn decrease_balance(
            &mut self, amount: u32) -> Result<i64, String> {
        if self.is_protected() && self.get_balance() < amount as i64 {
            Err(format!(  // ! Move this to transfer balance function?
                "This transaction is protected and will not allow \n\
                a charge(${}) greater than the account balance(${}).\n",
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
        amount: u32) -> Result<i64, String> {
    from_account.decrease_balance(amount)?;
    Ok(to_account.increase_balance(amount))
}
