use std::collections::HashMap;
use serde::{Deserialize, Serialize, Serializer};


#[derive(Serialize, Deserialize, Debug)]
// #[derive(Debug)]
pub enum RowValue {
    Id(String),
    Balance(i64),
    Pin(u16),
    Protected(bool),
}

// pub type Row = HashMap<String, RowValue>;


#[derive(Serialize, Deserialize, Debug)]
// #[derive(Debug)]
pub struct AccountData {
    pub rows: Vec<RowValue>,
}


impl AccountData {
    fn read_data() {}

    fn write_data() {}

    fn read_csv_data(filename: &str)
            -> Result<Vec<(String, i64, u16, bool)>, std::Error> {
                // ! Need to account for Err case
        let accounts_raw = std::fs::read_to_string("db.csv")?;
        // let mut accounts_raw = String::new();
        // if let Ok(accounts) = accounts_fs {
        //     accounts_raw = accounts;
        // }
        let mut accounts: Vec<(String, i64, u16, bool)> = Vec::new();
        let mut lines: Vec<(String, String, String, String)> = Vec::new();
        let mut headers: (String, String, String, String) = (
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        );
        for line in accounts_raw.split('\n') {
            let line = line.trim();
            if line.is_empty() {
                continue
            }
            let mut line_split = line.split(',');
            #[cfg(debug_assertions)]  // ! print on debug only
            println!("{}", line);  // ! print on debug only
            lines.push(
                (
                    line_split.next().unwrap().to_owned(),
                    line_split.next().unwrap().to_owned(),
                    line_split.next().unwrap().to_owned(),
                    line_split.next().unwrap().to_owned(),
                )
            );
        }

        for (i, line) in lines.into_iter().enumerate() {
            if i == 0 {
                headers = line.to_owned();
            } else {
                #[cfg(debug_assertions)]  // ! print on debug only
                println!("{:?}", line);  // ! print on debug only
                accounts.push((
                    line.0.to_owned(),
                    line.1.parse().unwrap(),
                    line.2.parse().unwrap(),
                    line.3.parse().unwrap(),
                ));
            }
        }
        Ok(accounts)
    }
}