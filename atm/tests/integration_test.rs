#[cfg(test)]

mod account_tests {

    use atm::account::Account;

    #[test]
    fn test_from_id() {}

    #[test]
    fn test_get_id() {
        let account = Account::new(String::from("1234567890"), 2000, 4425);
        assert_eq!(account.get_id(), String::from("1234567890"));
    }

    #[test]
    fn test_get_balance() {
        let account = Account::new(String::from("1234567890"), 2000, 4425);
        assert_eq!(*account.get_balance(), 2000);
    }

    #[test]
    fn test_set_balance_add() {
        let mut account = Account::new(String::from("1234567890"), 2000, 4425);
        account.increase_balance(300);
        assert_eq!(*account.get_balance(), 2300)
    }

    #[test]
    fn test_set_balance_subtract() {
        let mut account = Account::new(String::from("1234567890"), 2000, 4425);
        if let Ok(_) = account.decrease_balance(300) {
            assert_eq!(*account.get_balance(), 1700)
        }
    }
}

mod bank_tests {

    use atm::account::Account;
    use atm::bank::transfer_balance;

    #[test]
    fn test_transfer() -> Result<(), ()> {
        let mut account1 = Account::new(String::from("1234567890"), 2000, 4425);
        let mut account2 = Account::new(String::from("0987654321"), 2000, 8675);

        match transfer_balance(&mut account1, &mut account2, 500) {
            Ok(_) => {
                assert_eq!(*account1.get_balance(), 1500);
                assert_eq!(*account2.get_balance(), 2500);
                Ok(())
            }
            Err(_) => Err(()),
        }
    }

    #[test]
    fn test_transfer_fail() -> Result<(), ()> {
        let mut account1 = Account::new(String::from("1234567890"), 2000, 4425);
        let mut account2 = Account::new(String::from("0987654321"), 2000, 8675);

        match transfer_balance(&mut account1, &mut account2, 5000) {
            Err(_) => Ok(()),
            Ok(_) => Err(()),
        }
    }
}

mod data_tests {

    use std::fs;
    use atm::data;

    #[test]
    fn test_read_from_json() {
        let expected_output = [
            data::Row::new("8675309".to_string(), 2000, 1234, true),
            data::Row::new("4815162".to_string(), 2000, 4321, true),
            data::Row::new("5432121".to_string(), 2000, 9999, true),
            data::Row::new("3424551".to_string(), 2000, 9021, true),
        ];
        let mut accounts = data::AccountsData::new();
        if let Ok(()) = accounts.read_json_data("./tests/read_test.json") {
            assert_eq!(accounts.rows, expected_output);
        }
    }

    #[test]
    fn test_read_from_csv() {
        let expected_output = [
            data::Row::new("8675309".to_string(), 2000, 1234, true),
            data::Row::new("4815162".to_string(), 2000, 4321, true),
            data::Row::new("5432121".to_string(), 2000, 9999, true),
            data::Row::new("3424551".to_string(), 2000, 9021, true),
        ];
        let mut accounts = data::AccountsData::new();
        if let Ok(()) = accounts.read_csv_data("./tests/read_test.csv") {
            assert_eq!(accounts.rows, expected_output);
        }
    }

    #[test]
    fn test_write_to_json() {
        let expected_output = "[\
            {\"id\":\"8675309\",\"balance\":2000,\"pin\":1234,\"protected\":true},\
            {\"id\":\"4815162\",\"balance\":2000,\"pin\":4321,\"protected\":true},\
            {\"id\":\"5432121\",\"balance\":2000,\"pin\":9999,\"protected\":true},\
            {\"id\":\"3424551\",\"balance\":2000,\"pin\":9021,\"protected\":true}\
        ]"
        .to_string();
        let mut accounts = data::AccountsData::new();
        accounts.rows = vec![
            data::Row::new("8675309".to_string(), 2000, 1234, true),
            data::Row::new("4815162".to_string(), 2000, 4321, true),
            data::Row::new("5432121".to_string(), 2000, 9999, true),
            data::Row::new("3424551".to_string(), 2000, 9021, true),
        ];
        let test_filename = "./tests/write_test.json";
        if let Ok(json_data) = accounts.write_json_data(test_filename) {
            assert_eq!(json_data, expected_output);
        }
        fs::remove_file("./tests/write_test.json");
    }

    #[test]
    fn test_write_to_csv() {
        let expected_output = "\
            8675309,2000,1234,true\n\
            4815162,2000,4321,true\n\
            5432121,2000,9999,true\n\
            3424551,2000,9021,true\
        "
        .to_string();
        let mut accounts = data::AccountsData::new();
        accounts.rows = vec![
            data::Row::new("8675309".to_string(), 2000, 1234, true),
            data::Row::new("4815162".to_string(), 2000, 4321, true),
            data::Row::new("5432121".to_string(), 2000, 9999, true),
            data::Row::new("3424551".to_string(), 2000, 9021, true),
        ];
        let test_filename = "./tests/write_test.csv";
        if let Ok(json_data) = accounts.write_csv_data(test_filename) {
            assert_eq!(json_data, expected_output);
        }
        fs::remove_file("./tests/write_test.csv");
    }
}

mod user {}
