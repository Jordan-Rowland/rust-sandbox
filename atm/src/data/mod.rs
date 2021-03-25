use std::collections::HashMap;
use std::{error, fs};

const DATA: &str = "db.csv";

#[derive(Debug)]
// #[derive(Debug)]
pub enum RowValue {
    Id(String),
    Balance(i64),
    Pin(u16),
    Protected(bool),
}

pub type Row = HashMap<String, RowValue>;

#[derive(Debug)]
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

    pub fn write_csv_data() {}

    // pub fn read_json_data() -> Result<Vec<(String, i64, u16, bool)>, Box<dyn error::Error>> {
    pub fn read_json_data() -> Result<(), Box<dyn error::Error>> {
        let list_of_values: Vec<(String, i64, u16, bool)> = //list_of_string_values
        //let list_strings: Vec<String> =
         fs::read_to_string("db.json")?
            .split("},{")
            .map(|item| item.into())
        //     .collect();

        // let list_of_lists: Vec<Vec<String>> = list_strings
            .into_iter()
            .map(|item| item.split(",").map(|item| item.into()).collect::<Vec<_>>())
        //     .collect();

        // let list_of_string_values: Vec<Vec<String>> = list_of_lists
        //     .iter()
            .map(|vec_item| {
                vec_item
                    .iter()
                    .map(|inner_item| inner_item.split(":").collect::<Vec<&str>>()[1].to_owned())
                    .collect::<Vec<String>>()
            })
        //     .collect();

        // let list_of_values: Vec<(String, i64, u16, bool)> = list_of_string_values
        //     .iter()
            .map(|vec_item| {
                vec_item
                    .iter()
                    .map(|inner_item| {
                        inner_item
                            .replace("[", "")
                            .replace("]", "")
                            .replace("{", "")
                            .replace("}", "")
                            .replace("\"", "")
                    })
                    .collect()
            })
            .map(|vec_item: Vec<String>| {
                let mut vec_item = vec_item.iter();
                (
                    vec_item.next().unwrap().to_owned(),
                    vec_item.next().unwrap().parse().unwrap(),
                    vec_item.next().unwrap().parse().unwrap(),
                    vec_item.next().unwrap().parse().unwrap(),
                )
            })
            .collect();

        println!("{:#?}", list_of_values);
        // println!("{:?}", list_of_lists.len());

        Ok(())
    }

    pub fn write_json_data(
        accounts: &Vec<(String, i64, u16, bool)>,
    ) -> Result<(), Box<dyn error::Error>> {
        let acc_len = accounts.len();
        let fmt_rows: Vec<String> = accounts
            .iter()
            .map(|row| {
                format!(
                    "{{\
                    \"id\":\"{id}\",\
                    \"balance\":{balance},\
                    \"pin\":{pin},\
                    \"protected\":{protected}}}\
                ",
                    id = row.0,
                    balance = row.1,
                    pin = row.2,
                    protected = row.3,
                )
            })
            .enumerate()
            .map(|(i, mut row)| {
                if i != acc_len - 1 {
                    row.push(',');
                }
                row
            })
            .collect();

        let fmt_rows = format!("[{}]", fmt_rows.join(""));
        fs::write("db.json", fmt_rows)?;
        Ok(())
    }
}
