use std::{collections::HashMap, iter::FromIterator};
use std::{error, fs};

use crate::account::Account;

#[derive(Debug, PartialEq, Clone)]
pub struct Row {
    id: String,
    balance: i64,
    pin: u16,
    protected: bool,
}

impl Row {
    pub fn new(id: String, balance: i64, pin: u16, protected: bool) -> Self {
        Row {
            id,
            balance,
            pin,
            protected,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_balance(&self) -> &i64 {
        &self.balance
    }

    pub fn get_pin(&self) -> &u16 {
        &self.pin
    }

    pub fn get_protected(&self) -> &bool {
        &self.protected
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AccountsData {
    pub rows: Vec<Row>,
}

impl FromIterator<Row> for AccountsData {
    fn from_iter<I: IntoIterator<Item = Row>>(iter: I) -> Self {
        let mut c = AccountsData::new();

        for i in iter {
            c.rows.push(i);
        }

        c
    }
}

impl AccountsData {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn update(&mut self, new_row: Row) -> Row {
        let new_rows = self.rows.clone();
        let mut new_rows = new_rows
            .iter()
            .filter(|row| row.get_id() != new_row.get_id())
            .map(|row| row.clone())
            .collect::<Vec<Row>>();
        new_rows.push(new_row.clone());
        self.rows = new_rows.clone();
        new_row
    }

    fn push(&mut self, row: Row) {
        &self.rows.push(row);
    }

    pub fn read_csv_data(&mut self, filename: &str) -> Result<(), Box<dyn error::Error>> {
        let rows = fs::read_to_string(filename)?
            .lines()
            .enumerate()
            .filter(|(_i, line)| !line.is_empty())
            .filter(|(i, _line)| *i != 0)
            .map(|(_i, line)| line.split(','))
            .map(|mut line| {
                Row {
                    // Parse and unwrap elements into their types in a row
                    id: line.next().unwrap().to_owned(),
                    balance: line.next().unwrap().parse().unwrap(),
                    pin: line.next().unwrap().parse().unwrap(),
                    protected: line.next().unwrap().parse().unwrap(),
                }
            })
            .collect::<Vec<Row>>();
        self.rows = rows;
        Ok(())
    }

    pub fn write_json_data(&self, filename: &str) -> Result<String, Box<dyn error::Error>> {
        let acc_len = self.rows.len();
        let fmt_rows: Vec<String> = self
            .rows
            .iter()
            .map(|row| {
                format!(
                    "{{\
                    \"id\":\"{id}\",\
                    \"balance\":{balance},\
                    \"pin\":{pin},\
                    \"protected\":{protected}}}\
                ",
                    id = row.id,
                    balance = row.balance,
                    pin = row.pin,
                    protected = row.protected,
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
        fs::write(filename, &fmt_rows)?;
        Ok(fmt_rows)
    }

    pub fn read_json_data(&mut self, filename: &str) -> Result<(), Box<dyn error::Error>> {
        let rows: Vec<Row> = fs::read_to_string(filename)?
            .split("},{") // Split by comma and braces between JSON objects
            .map(|item| item.into())
            .into_iter()
            // Split individual vec items to their inner elements
            .map(|item: String| item.split(",").map(|item| item.into()).collect::<Vec<_>>())
            .map(|vec_item: Vec<String>| {
                vec_item
                    .iter()
                    // Split inner elements by key and value
                    .map(|inner_item| inner_item.split(":").collect::<Vec<&str>>()[1].to_owned())
                    .collect::<Vec<String>>()
            })
            .map(|vec_item| {
                vec_item
                    .iter()
                    // Replace brackets and quotes on the raw string elements
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
                Row {
                    // Parse and unwrap elements into their types in a row
                    id: vec_item.next().unwrap().to_owned(),
                    balance: vec_item.next().unwrap().parse().unwrap(),
                    pin: vec_item.next().unwrap().parse().unwrap(),
                    protected: vec_item.next().unwrap().parse().unwrap(),
                }
            })
            .collect();
        self.rows = rows;
        Ok(())
    }

    pub fn write_csv_data(&self, filename: &str) -> Result<String, Box<dyn error::Error>> {
        let acc_len = self.rows.len();
        let fmt_headers = "id,balance,pin,protected\n";
        let fmt_rows: String = self
            .rows
            .iter()
            .map(|row| {
                format!(
                    "{id},{balance},{pin},{protected}",
                    id = row.id,
                    balance = row.balance,
                    pin = row.pin,
                    protected = row.protected,
                )
            })
            .enumerate()
            .map(|(i, mut row)| {
                if i != acc_len - 1 {
                    row.push('\n');
                }
                row
            })
            .collect();

        fs::write(filename, format!("{}{}", fmt_headers, fmt_rows))?;
        Ok(fmt_rows)
    }
}
