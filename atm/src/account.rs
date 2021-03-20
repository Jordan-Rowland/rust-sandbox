#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]


#[derive(Debug)]
pub struct Account {
    id: String,
    balance: i64,
    pin: u16,
    protected: bool,
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

    pub fn is_protected(&self) -> bool {
        self.protected
    }

    pub fn set_balance(&mut self, amount: u32, transaction_type: TransactionType) -> i64 {
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

    pub fn from_id(id: &str, accounts: &[(String, i64, u16, bool)]) -> Option<Self> {
        match accounts
        .iter()
        .find(|(item_id, item_balance, item_pin, protected)| item_id == id) {
            Some((id, balance, pin, protected)) => {
                Some(Self {
                    id: id.to_owned(),
                    balance: *balance,
                    pin: *pin,
                    protected: *protected,
                })
            }
            None => None
        }
    }
}
