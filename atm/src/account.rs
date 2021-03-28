#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use crate::data::AccountsData;

#[derive(Debug)]
pub struct Account {
    id: String,
    balance: i64,
    pin: u16,
    protected: bool,
}

pub enum TransactionType {
    Add,
    Subtract,
}

impl Account {
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_balance(&self) -> &i64 {
        &self.balance
    }

    pub fn is_protected(&self) -> &bool {
        &self.protected
    }

    fn set_balance(&mut self, amount: u32, transaction_type: TransactionType) -> i64 {
        match transaction_type {
            TransactionType::Add => self.balance += amount as i64,
            TransactionType::Subtract => self.balance -= amount as i64,
        }
        self.balance
    }

    pub fn new(id: String, balance: i64, pin: u16) -> Self {
        Self {
            id,
            balance,
            pin,
            protected: true,
        }
    }

    pub fn from_id(id: &str, accounts: &AccountsData) -> Option<Self> {
        match accounts.rows.iter().find(|row| row.get_id() == id) {
            Some(row) => Some(Self {
                id: id.to_owned(),
                balance: *row.get_balance(),
                pin: *row.get_pin(),
                protected: *row.get_protected(),
            }),
            None => None,
        }
    }

    pub fn increase_balance(&mut self, amount: u32) -> i64 {
        self.set_balance(amount, TransactionType::Add)
    }

    pub fn decrease_balance(&mut self, amount: u32) -> Result<i64, String> {
        if *self.is_protected() && *self.get_balance() < amount as i64 {
            Err(format!(
                // ! Move this to transfer balance function?
                "This transaction is protected and will not allow \n\
                    a charge(${}) greater than the account balance(${}).\n",
                amount,
                self.get_balance()
            ))
        } else {
            Ok(self.set_balance(amount, TransactionType::Subtract))
        }
    }
}
