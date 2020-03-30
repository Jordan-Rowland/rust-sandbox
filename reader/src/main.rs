use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

struct Data {
    headers: Vec<String>,
    rows: Vec<HashMap<String, String>>,
    filename: String,
    rows_len: u32,
}

impl Data {
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
            rows_len: rows.len() as u32,
        })
    }

    pub fn new() -> Data {
        Data {
            headers: Vec::new(),
            rows: Vec::new(),
            filename: String::new(),
            rows_len: 0,
        }
    }

    pub fn add_row(&mut self, row: HashMap<String, String>) {
        let mut proceed = true;
        for header in &self.headers {
            if !row.contains_key(header) {
                proceed = false;
            }
        }
        if proceed {
            self.rows.push(row);
            if let Ok(()) = self.write_csv(None) {
                println!("CSV updated");
            } else {
                self.rows.pop();
            }
        } else {
            println!("Some headers are not valid or missing")
        }
    }

    pub fn edit_row(&mut self, index: usize, row: HashMap<String, String>) {
        self.rows[index] = row;
    }

    pub fn drop_row(&mut self, index: usize) {
        self.rows.remove(index);
    }

    fn write_csv(&self, mut filename: Option<String>) -> std::io::Result<()> {
        if let None = filename {
            filename = Some(self.filename.clone());
        }
        let mut file = File::create(filename.unwrap().to_string())?;
        for i in 0..self.headers.len() {
            if i == self.headers.len() - 1 {
                writeln!(file, "{}", &self.headers[i])?;
            } else {
                write!(file, "{},", &self.headers[i])?;
            }
        }
        for row in &self.rows {
            for header_index in 0..self.headers.len() {
                let value = row.get(&self.headers[header_index]).unwrap().to_string();
                if header_index == self.headers.len() - 1 {
                    writeln!(file, "{}", value)?;
                } else {
                    write!(file, "{},", value)?;
                }
            }
        }
        Ok(())
    }
}

fn main() {
    let mut data = Data::open_file("db.csv").expect("Expecting data here");
    let mut it = true;
    while it {
        let mut name_input = String::new();
        println!("Please enter a name");
        io::stdin()
            .read_line(&mut name_input)
            .expect("Name not read");
        let mut email_input = String::new();
        println!("Please enter an email");
        io::stdin()
            .read_line(&mut email_input)
            .expect("Name not read");
        let mut job_title_input = String::new();
        println!("Please enter a job title");
        io::stdin()
            .read_line(&mut job_title_input)
            .expect("Name not read");
        let mut salary_input = String::new();
        println!("Please enter a salary");
        io::stdin()
            .read_line(&mut salary_input)
            .expect("Name not read");

        let mut h = HashMap::new();
        h.insert("name".to_string(), name_input.trim().to_lowercase());
        h.insert("email".to_string(), email_input.trim().to_lowercase());
        h.insert(
            "job_title".to_string(),
            job_title_input.trim().to_lowercase(),
        );
        h.insert("salary".to_string(), salary_input.trim().to_lowercase());
        data.add_row(h);

        data.write_csv(None);
        it = false;
    }

    // println!("{:?}", data.headers);

    // println!("{:?}", data.rows);

    // for row in &data.rows {
    //     println!("{:?}", row);
    // }

    // h.insert("name".to_string(), "jimjam".to_string());
    // h.insert("email".to_string(), "jimjam@email.com".to_string());
    // h.insert("job_title".to_string(), "poopman".to_string());
    // h.insert("salary".to_string(), "20000".to_string());
}
