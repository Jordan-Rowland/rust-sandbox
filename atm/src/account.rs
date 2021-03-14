#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]


pub struct Account {
    id: String,
    balance: i64,
    pin: u8
}

pub enum TransactionType {
    Add,
    Subtract
}

impl Account {

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_balance(&self) -> i64 {
        self.balance
    }

    pub fn set_balance(&mut self, amount: i64, transaction_type: TransactionType) -> i64 {
        match transaction_type {
            TransactionType::Add => self.balance += amount,
            TransactionType::Subtract => self.balance -= amount,
        }
        self.balance
    }

    pub fn new(id: String, balance: i64, pin: u8) -> Self {
        Self {
            id,
            balance,
            pin
        }
    }

    pub fn from_id(id: &str, accounts: &[(String, i64, u8)]) -> Option<Self> {
        match accounts
        .iter()
        .find(|(item_id, item_balance, item_pin)| item_id == id) {
            Some((id, balance, pin)) => {
                Some(Self {
                    id: id.to_owned(),
                    balance: *balance,
                    pin: *pin
                })
            }
            None => None
        }
    }
}
