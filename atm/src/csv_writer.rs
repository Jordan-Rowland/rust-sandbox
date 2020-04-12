use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Data {
    filename: String,
    headers: Vec<String>,
    rows: Vec<Row>,
    pub rows_len: usize,
}

pub enum Filename {
    Existing,
    New(String),
}

pub type Row = HashMap<String, String>;

// New and initialization methods
impl Data {
    pub fn new(filename: &str, headers: Vec<String>) -> Self {
        Self {
            filename: filename.into(),
            headers,
            rows: Vec::new(),
            rows_len: 0,
        }
    }

    pub fn from_rows(rows: Vec<Row>) -> Self {
        let mut keys = Vec::new();
        let mut owned_rows = Vec::new();
        for key in rows[0].keys() {
            keys.push(key.to_string());
        }
        for row in rows {
            owned_rows.push(row.to_owned());
        }
        Self {
            filename: "".to_string(),
            headers: keys,
            rows: owned_rows.clone(),
            rows_len: owned_rows.len().to_owned(),
        }
    }
}

// Getters
impl Data {
    pub fn get_headers(&self) -> &Vec<String> {
        &self.headers
    }

    pub fn get_rows(&self) -> &Vec<Row> {
        &self.rows
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
    }

    pub fn get_column(&self, column: &str) -> Option<Vec<String>> {
        if !self.headers.contains(&column.to_string()) {
            return Option::None;
        }
        let mut found_columns = Vec::new();
        for row in &self.rows {
            let c = row.get(column).unwrap();
            found_columns.push(c.into());
        }
        Some(found_columns)
    }

    pub fn find_rows_by_column(&self, column: &str, value: &str) -> std::io::Result<Vec<Row>> {
        Ok(self
            .rows
            .clone()
            .into_iter()
            .filter(|row| {
                row.get(&column.to_lowercase())
                    .unwrap()
                    .contains(&value.to_lowercase())
            })
            .collect())
    }
}

// Setters
impl Data {
    pub fn add_row(&mut self, row: Row) {
        let mut proceed = true;
        for header in &self.headers {
            if !row.contains_key(&header.to_lowercase()) {
                proceed = false;
            }
        }
        if proceed {
            self.rows.push(row);
            self.rows_len += 1;
            if self.filename.len() > 1 {
                if let Ok(()) = self.write_csv(Filename::Existing) {
                    println!("File updated: {}", self.filename);
                } else {
                    println!("Couldn't write to file: {}", self.filename);
                    self.rows.pop();
                    self.rows_len -= 1;
                }
            }
        } else {
            println!("Some headers are not valid or missing");
        }
    }

    pub fn edit_row(&mut self, index: usize, row: Row) {
        self.rows[index] = row;
    }

    pub fn drop_row(&mut self, index: usize) {
        self.rows.remove(index);
        self.rows_len -= 1;
    }

    pub fn set_filename(&mut self, new_filename: &str) {
        self.filename = new_filename.into()
    }

    pub fn calc_rows_len(&mut self) {
        self.rows_len = self.rows.len();
    }
}

// File io
impl Data {
    pub fn open_file(filename: &str) -> std::io::Result<Self> {
        let mut contents = String::new();
        File::open(filename)?.read_to_string(&mut contents)?;
        let mut contents_iter = contents.split("\n");
        let header_iter = contents_iter.next().unwrap().split(",");
        let mut headers = Vec::new();
        for header in header_iter {
            headers.push(header.to_string().to_lowercase())
        }
        let mut rows = Vec::new();
        for row in contents_iter {
            let mut row_hashmap = HashMap::new();
            if row.len() != 0 {
                let mut row_split = row.split(",");
                for header in headers.clone() {
                    row_hashmap
                        .insert(header, row_split.next().unwrap().to_string().to_lowercase());
                }
                rows.push(row_hashmap);
            }
        }
        Ok(Self {
            headers,
            rows: rows.clone(),
            filename: String::from(filename),
            rows_len: rows.len(),
        })
    }

    pub fn write_csv(&self, filename: Filename) -> std::io::Result<()> {
        let string_filename: String;
        match filename {
            Filename::New(filename) => string_filename = filename.to_owned(),
            Filename::Existing => string_filename = self.filename.to_owned(),
        }
        let mut file = File::create(string_filename)?;
        for i in 0..self.headers.len() {
            if i == self.headers.len() - 1 {
                writeln!(file, "{}", &self.headers[i].to_lowercase())?;
            } else {
                write!(file, "{},", &self.headers[i].to_lowercase())?;
            }
        }
        for row in &self.rows {
            for header_index in 0..self.headers.len() {
                let value = row.get(&self.headers[header_index]).unwrap().to_string();
                if header_index == self.headers.len() - 1 {
                    writeln!(file, "{}", value.to_lowercase())?;
                } else {
                    write!(file, "{},", value.to_lowercase())?;
                }
            }
        }
        Ok(())
    }
}
