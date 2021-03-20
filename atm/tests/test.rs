// #[cfg(test)]
// mod tests {
//     // use super::*;

//     #[test]
//     fn internal() {
//         assert_eq!(4, 2 + 2);
//     }
// }


#[cfg(test)]
mod account_tests {

    use atm::account::Account;
    use atm::account::TransactionType;

    #[test]
    fn test_from_id() {}

    #[test]
    fn test_get_id() {
        let account = Account::new(
            String::from("1234567890"),
            2000,
            4425
        );
        assert_eq!(account.get_id(), String::from("1234567890"));
    }

    #[test]
    fn test_get_balance() {
        let account = Account::new(
            String::from("1234567890"),
            2000,
            4425
        );
        assert_eq!(account.get_balance(), 2000);
    }

    #[test]
    fn test_set_balance_add() {
        let mut account = Account::new(
            String::from("1234567890"),
            2000,
            4425
        );
        account.set_balance(300, TransactionType::Add);
        assert_eq!(account.get_balance(), 2300)
    }

    #[test]
    fn test_set_balance_subtract() {
        let mut account = Account::new(
            String::from("1234567890"),
            2000,
            4425
        );
        account.set_balance(300, TransactionType::Subtract);
        assert_eq!(account.get_balance(), 1700)
    }

}


mod bank_tests {
    use atm::account::Account;
    use atm::bank::transfer_balance;
    // use atm::bank::TransactionSecurity;

    #[test]
    fn test_transfer() -> Result<(), ()> {
        let mut account1 = Account::new(
            String::from("1234567890"),
            2000,
            4425
        );
        let mut account2 = Account::new(
            String::from("0987654321"),
            2000,
            8675
        );

        match transfer_balance(
            &mut account1,
            &mut account2,
            500
        ) {
            Ok(_) => {
                assert_eq!(account1.get_balance(), 1500);
                assert_eq!(account2.get_balance(), 2500);
                Ok(())
            },
            Err(_) => Err(())
        }
    }

    #[test]
    fn test_transfer_fail() -> Result<(), ()> {
        let mut account1 = Account::new(
            String::from("1234567890"),
            2000,
            4425
        );
        let mut account2 = Account::new(
            String::from("0987654321"),
            2000,
            8675
        );

        match transfer_balance(
            &mut account1,
            &mut account2,
            5000
        ) {
            Err(_) => Ok(()),
            Ok(_) => Err(())
        }
    }

}


mod client_tests {

    #[test]
    fn test_input() {}

    #[test]
    fn test_num_input() {}

}