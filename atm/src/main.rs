#![allow(dead_code)]

use std::io;

mod csv_writer;

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    pin: u16,
}

fn initialize() -> csv_writer::Data {
    csv_writer::Data::new(
        "db.csv",
        vec!["id".to_string(), "balance".to_string(), "pin".to_string()],
    )
}

fn initialize_existing(filename: &str) -> std::io::Result<csv_writer::Data> {
    let d = csv_writer::Data::open_file(filename)?;
    Ok(d)
}

fn authenticate(d: &csv_writer::Data) -> Option<Account> {
    let mut pin_input = String::new();
    println!("ENTER PIN:");
    io::stdin()
        .read_line(&mut pin_input)
        .expect("Could not read line");
    let _r = d.exact_find_rows_by_column("pin", &pin_input.trim());
    let current_user: Account;
    if let Some(r) = _r {
        current_user = Account::init_from_row(&r[0]).expect("Cound't initialize from rows;");
        Some(current_user)
    } else {
        None
    }
}

fn find_account_by_id(d: &csv_writer::Data) -> Option<csv_writer::Row> {
    let mut id_input = String::new();
    println!("ENTER ID:");
    io::stdin()
        .read_line(&mut id_input)
        .expect("Could not read line");
    let _r = d.exact_find_rows_by_column("id", &id_input.trim());
    let found_user: csv_writer::Row;
    if let Some(r) = _r {
        found_user = r[0].to_owned();
        Some(found_user)
    } else {
        None
    }
}

impl Account {
    fn new(id: u32, balance: i32, pin: u16) -> Self {
        Self { id, balance, pin }
    }

    fn init_from_row(row: &csv_writer::Row) -> Result<Self, std::io::Error> {
        Ok(Self {
            id: row.get("id").unwrap().parse().unwrap(),
            balance: row.get("balance").unwrap().parse().unwrap(),
            pin: row.get("pin").unwrap().parse().unwrap(),
        })
    }

    fn display_balance(&self) -> String {
        format!("{}", self.balance as f64 / 100.00)
    }

    fn deposit(&mut self, amount: f64) -> i32 {
        let amount: i32 = (amount * 100.00) as i32;
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: f64) -> i32 {
        let amount: i32 = (amount * 100.00) as i32;
        self.balance -= amount;
        self.balance
    }

    // fn transfer_money(&mut self, account_id: u32, amount: u32) {}
}

fn main() {
    // use std::collections::HashMap;

    // let mut h = HashMap::new();
    // h.insert("id".to_string(), "2".to_string());
    // h.insert("balance".to_string(), "25000".to_string());
    // h.insert("pin".to_string(), "1233".to_string());

    // let mut d = initialize();
    let _d = initialize_existing("db.csv");
    let d = _d.unwrap();
    // d.add_row(h);
    // d.write_csv(csv_writer::Filename::Existing);

    // let current_user = authenticate(&d);

    // println!("{:?}", current_user);

    let _a = find_account_by_id(&d);
    let mut di: Option<usize> = None;
    if let Some(a) = _a {
        di = d.get_row_index(a);
    } else {
        println!("FUCK");
    }
    if let Some(i) = di {
        println!("line 118: {:?}", d.get_rows()[i]);
    } else {
        println!("line 120: OOPS");
    }

    // loop {
    //     println!("Select an action:\n\nD) Deposit\nW) Withdraw\nQ) Quit");
    //     let mut action = String::new();
    //     io::stdin()
    //         .read_line(&mut action)
    //         .expect("Could not read line");
    //     match action.trim().to_lowercase().as_str() {
    //         "d" => println!("Deposit\n"),
    //         "w" => println!("withdraw\n"),
    //         "q" => break,
    //         _ => (),
    //     }
    // }
}
