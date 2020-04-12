use std::io;

mod csv_writer;
// use csv_writer;

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    pin: u16,
}

fn initialize() -> csv_writer::Data {
    let mut d = csv_writer::Data::new(
        "db.csv",
        vec!["id".to_string(), "balance".to_string(), "pin".to_string()],
    );
    d
}

fn initialize_existing(filename: &str) -> std::io::Result<csv_writer::Data> {
    let d = csv_writer::Data::open_file(filename)?;
    Ok(d)
}

fn authenticate(d: csv_writer::Data) -> Option<Account> {
    let mut pin_input = String::new();
    println!("ENTER PIN:");
    io::stdin()
        .read_line(&mut pin_input)
        .expect("Could not read line");
    // TODO: Need to fix the specifics of this function for exact match
    let _r = d.find_rows_by_column("pin", &pin_input.trim());
    let mut current_user: Account;
    if let Ok(r) = _r {
        current_user = Account::init_from_row(&r[0]).expect("Cound't do it");
        Some(current_user)
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

    fn withraw(&mut self, amount: f64) -> i32 {
        let amount: i32 = (amount * 100.00) as i32;
        self.balance -= amount;
        self.balance
    }

    fn transfer_money(&mut self, account_id: u32) {}
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

    let current_user = authenticate(d);

    println!("{:?}", current_user);

    loop {
        println!("Select an action:\n\nD) Deposit\nW) Withdraw\nQ) Quit");
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Could not read line");
        match action.trim().to_lowercase().as_str() {
            "d" => println!("Deposit\n"),
            "w" => println!("withdraw\n"),
            "q" => break,
            _ => (),
        }
        // break;
    }
}
