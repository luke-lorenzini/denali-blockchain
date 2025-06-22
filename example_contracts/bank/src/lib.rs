use std::{ffi::c_void, collections::HashMap, sync::{
    Arc,
    Mutex
}};

// use tokio::sync::Mutex;
use async_trait::async_trait;
use denali::{
    storage::State,
    types::{RawTraitObject, Thing, H256},
};
use rust_decimal::{Decimal, dec};
use serde::Deserialize;
use serde_json::Result;

// #[unsafe(no_mangle)]
// pub extern "C" fn create_thing() -> *mut dyn Thing {
//     println!("Creating bank");
//     let bank = Bank::new(vec![]);
//     let boxed_bank = Box::new(bank);
//     Box::into_raw(boxed_bank)
// }

#[unsafe(no_mangle)]
pub extern "C" fn create_thing() -> *mut c_void {
    println!("Creating bank");
    let boxed_bank: Box<dyn Thing> = Box::new(Bank::new(vec![]));
    let raw_fat_ptr = Box::into_raw(boxed_bank);
    unsafe {
        let (data_ptr, vtable_ptr): (*mut c_void, *mut c_void) = std::mem::transmute(raw_fat_ptr);

        let boxed_raw_trait_object = Box::new(RawTraitObject {
            data_ptr,
            vtable_ptr,
        });
        Box::into_raw(boxed_raw_trait_object) as *mut c_void
    }
}

#[derive(Debug)]
pub struct Bank {
    accounts: HashMap<u32, Account>,
}

#[async_trait]
impl Thing for Bank {
    fn name(&self) -> &'static str {
        "bank"
    }

    async fn run(&self, payload: &str, _state: Arc<Mutex<State>>) -> Result<H256> {
        bank_program(payload)?;
        Ok(H256::default())
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
            println!("SUCCESSFUL TRANSFER");
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
