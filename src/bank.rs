use rust_decimal::{Decimal, dec};
use serde::Deserialize;
use serde_json::Result;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Bank {
    accounts: HashMap<u32, Account>,
}

impl Bank {
    pub fn new(accounts: Vec<Account>) -> Self {
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
pub struct Account {
    #[allow(dead_code)]
    name: String,
    balance: Decimal,
}

impl Account {
    pub fn new(name: String, balance: Decimal) -> Self {
        Self { name, balance }
    }
}

pub fn bank_program(payload: &str) -> Result<()> {
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

    fn setup() -> String {
        r#"
        {
            "payer": 0,
            "payee": 1,
            "amount": 213.7
        }"#
        .into()
    }

    #[test]
    fn test_bank_program() {
        let payload = setup();
        let _res = bank_program(&payload).unwrap();
    }
}
