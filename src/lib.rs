use serde_json::Result;

mod bank;
pub mod transfer;
mod vote;

pub trait Thing {
    fn verify(&self) -> Result<bool>;
    fn run(&self, payload: &str) -> Result<()>;
}

pub fn parse<T: Thing>(program: T, payload: &str) -> Result<()> {
    if program.verify()? == true {
        program.run(payload)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    // use super::*;

    fn _setup() -> String {
        r#"
        {
            "payer": 0,
            "payee": 1,
            "amount": 213.7
        }"#
        .into()
    }

    // #[test]
    // fn test_parse() {
    //     let _res = parse();
    // }
}
