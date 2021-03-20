struct Config<'a> {
    filename: &'a str,
    query: &'a str,
}

impl Config {
    fn from_params(params: &[&str]) -> Self {
        Config {
            query: &params[1],
            filename: &params[2],
        }
    }
}