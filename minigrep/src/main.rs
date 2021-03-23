use std::error::Error;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_params(env::args()).unwrap();
    let contents = fs::read_to_string(&config.filename)?;

    let found_contents: Vec<(usize, &str)> = contents
        .lines()
        .enumerate()
        .filter(|(_i, line)| line.to_lowercase().contains(&config.query))
        .collect();

    for (i, item) in found_contents.iter() {
        println!("{:>3}:  {}", i + 1, item)
    }
    Ok(())
}

struct Config<> {
    filename: String,
    query: String,
}

impl<'a> Config<> {
    fn from_params(mut params: env::Args) -> Result<Self, &'static str> {
        params.next();

        let filename = match params.next() {
            Some(arg) => arg,
            None => return Err("Did not receive a file name.")
        };

        let query = match params.next() {
            Some(arg) => arg,
            None => return Err("Did not receive a query string.")
        };

        Ok(Self {
            filename,
            query,
        })
    }
}
