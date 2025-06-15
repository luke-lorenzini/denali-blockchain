use std::collections::HashMap;

use rust_decimal::{Decimal, dec};
use serde::Deserialize;
use serde_json::Result;

use denali::{storage::State, types::Thing};

#[derive(Debug)]
pub struct Bank {
    accounts: HashMap<u32, Account>,
}

impl Thing for Bank {
    fn run(&self, payload: &str, _state: &State) -> Result<()> {
        bank_program(payload)?;
        Ok(())
    }

    fn verify(&self) -> Result<bool> {
        Ok(true)
    }
}

impl Bank {
    fn new(accounts: Vec<Account>) -> Self {
        let mut local_accounts = HashMap::new();

        for (account_number, account) in accounts.into_iter().enumerate() {
            local_accounts.insert(account_number as u32, account);
        }

        Self {
            accounts: local_accounts,
        }
    }
}

#[derive(Debug)]
struct Account {
    #[allow(dead_code)]
    name: String,
    balance: Decimal,
}

impl Account {
    fn new(name: String, balance: Decimal) -> Self {
        Self { name, balance }
    }
}

fn bank_program(payload: &str) -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct BankTransfer {
        payer: u32,
        payee: u32,
        amount: Decimal,
    }

    let payload: BankTransfer = serde_json::from_str(payload)?;
    println!("payload: {payload:?}");

    let account1 = Account::new("user1".into(), dec!(400));
    let account2 = Account::new("user2".into(), Decimal::ZERO);
    let accounts = vec![account1, account2];
    let bank = Bank::new(accounts);
    println!("bank: {bank:?}");

    if bank.accounts.contains_key(&payload.payee) && bank.accounts.contains_key(&payload.payer) {
        let account_details = bank.accounts.get(&payload.payer).expect("Already checked");
        if account_details.balance >= payload.amount {
        } else {
            todo!("NSF")
        }
    } else {
        todo!("Missing an account")
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> (Bank, String) {
        let payload = r#"
        {
            "payer": 0,
            "payee": 1,
            "amount": 213.7
        }"#
        .into();
        let account1 = Account::new("user1".into(), dec!(400));
        let account2 = Account::new("user2".into(), Decimal::ZERO);
        let accounts = vec![account1, account2];
        let bank = Bank::new(accounts);
        (bank, payload)
    }

    #[test]
    fn test_bank_program() {
        let (_, payload) = setup();
        let _res = bank_program(&payload).unwrap();
    }

    #[test]
    fn test_run() {
        // let (bank, payload) = setup();
        // let _res = bank.run(&payload).unwrap();
    }
}
