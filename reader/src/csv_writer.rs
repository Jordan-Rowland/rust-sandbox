use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub struct Data {
    filename: String,
    headers: Vec<String>,
    rows: Vec<HashMap<String, String>>,
    rows_len: usize,
}

impl Data {
    pub fn new(filename: String, headers: Vec<String>) -> Data {
        Data {
            filename,
            headers,
            rows: Vec::new(),
            rows_len: 0,
        }
    }

    pub fn get_headers(&self) -> &Vec<String> {
        &self.headers
    }

    pub fn get_rows(&self) -> &Vec<HashMap<String, String>> {
        &self.rows
    }

    pub fn add_row(&mut self, row: HashMap<String, String>) {
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
                if let Ok(()) = self.write_csv(None) {
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

    pub fn edit_row(&mut self, index: usize, row: HashMap<String, String>) {
        self.rows[index] = row;
    }

    pub fn drop_row(&mut self, index: usize) {
        self.rows.remove(index);
        self.rows_len -= 1;
    }

    pub fn calc_rows_len(&mut self) {
        self.rows_len = self.rows.len();
    }

    pub fn open_file(filename: &str) -> std::io::Result<Data> {
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
        Ok(Data {
            headers,
            rows: rows.clone(),
            filename: String::from(filename),
            rows_len: rows.len(),
        })
    }

    fn write_csv(&self, mut filename: Option<String>) -> std::io::Result<()> {
        match filename {
            Some(_) => (),
            None => filename = Some(self.filename.clone()),
        }
        let mut file = File::create(filename.unwrap().to_string())?;
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

    pub fn from_rows(rows: Vec<&HashMap<String, String>>) -> Data {
        let mut keys = Vec::new();
        let mut owned_rows = Vec::new();
        for key in rows[0].keys() {
            keys.push(key.to_string());
        }
        for row in rows {
            owned_rows.push(row.to_owned());
        }
        Data {
            filename: "".to_string(),
            headers: keys,
            rows: owned_rows.clone(),
            rows_len: owned_rows.len().to_owned(),
        }
    }
}
