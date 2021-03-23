use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use std::{error, fs};

const DATA: &str = "db.csv";

#[derive(Serialize, Deserialize, Debug)]
// #[derive(Debug)]
pub enum RowValue {
    Id(String),
    Balance(i64),
    Pin(u16),
    Protected(bool),
}

pub type Row = HashMap<String, RowValue>;

#[derive(Serialize, Deserialize, Debug)]
// #[derive(Debug)]
pub struct AccountsData {
    // pub rows: Vec<RowValue>,
    pub rows: Vec<Row>,
}

impl AccountsData {
    pub fn read_csv_data() -> Result<Vec<(String, i64, u16, bool)>, Box<dyn error::Error>> {
        Ok(fs::read_to_string("./src/data/csv/db.csv")?
            .lines()
            .enumerate()
            .filter(|(_i, line)| !line.is_empty())
            .filter(|(i, _line)| *i != 0)
            .map(|(_i, line)| line.split(','))
            .map(|mut line| {
                (
                    line.next().unwrap().to_owned(),
                    line.next().unwrap().parse().unwrap(),
                    line.next().unwrap().parse().unwrap(),
                    line.next().unwrap().parse().unwrap(),
                )
            })
            .collect::<Vec<(String, i64, u16, bool)>>())
    }

    pub fn write_data_json(
        accounts: &Vec<(String, i64, u16, bool)>,
    ) -> Result<(), Box<dyn error::Error>> {
        let acc_len = accounts.len();
        let fmt_rows: Vec<String> = Vec::with_capacity(acc_len);
        for (i, row) in accounts.iter().enumerate() {
            let mut str_row = format!(
                "{{\
                \"id\":{id:?},\
                \"balance\":{balance:?},\
                \"pin\":{pin:?},\
                \"protected\":{protected:?},}}\
            ",
                id = row.0,
                balance = row.1,
                pin = row.2,
                protected = row.3,
            );
            if i != acc_len - 1 {
                str_row.push(',');
            }
            println!("{}", str_row);
            // fmt_rows.push(str_row);
        }
        // fs::write("./src/data/db.json", fmt_rows)?;
        Ok(())
    }
}
