#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use crate::account::Account;
use crate::account::TransactionType;

pub fn transfer_balance(
    from_account: &mut Account,
    to_account: &mut Account,
    amount: u32,
) -> Result<i64, String> {
    from_account.decrease_balance(amount)?;
    Ok(to_account.increase_balance(amount))
}
