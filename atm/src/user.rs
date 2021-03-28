struct User {
    id: String,
    pub name: String,
    pin: u16,
    accounts: Vec<String>,
}

impl User {
    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_pin(&self) -> &u16 {
        &self.pin
    }

    fn get_accounts(&self) -> &Vec<String> {
        &self.accounts
    }
}
