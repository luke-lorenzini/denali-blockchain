use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use denali::{storage::State, types::Thing};
use macros::generate_create_thing;
use rust_decimal::{Decimal, dec};
use semver::Version;
use serde::Deserialize;
use serde_json::Result;
use tokio::sync::Mutex;

#[generate_create_thing(args(vec![]))]
#[derive(Clone, Debug)]
pub struct Bank {
    accounts: HashMap<u32, Account>,
}

#[async_trait]
impl Thing for Bank {
    fn name(&self) -> &'static str {
        "bank"
    }

    fn version(&self) -> Version {
        Version::parse("0.1.0").unwrap()
    }

    async fn run(&self, payload: &str, _state: Arc<Mutex<State>>) -> Result<(bool, Vec<String>)> {
        let logs = bank_program(payload)?;
        Ok((true, logs))
    }

    fn verify(&self) -> Result<bool> {
        Ok(true)
    }
}

impl Bank {
    fn new(accounts: Vec<Account>) -> Self {
        let mut local_accounts = HashMap::new();

        for (account_number, account) in accounts.into_iter().enumerate() {
            local_accounts.insert(u32::try_from(account_number).unwrap(), account);
        }

        Self {
            accounts: local_accounts,
        }
    }
}

#[derive(Clone, Debug)]
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

fn bank_program(payload: &str) -> Result<Vec<String>> {
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
            println!("SUCCESSFUL TRANSFER");
        } else {
            todo!("NSF")
        }
    } else {
        todo!("Missing an account")
    }

    Ok(vec![])
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
