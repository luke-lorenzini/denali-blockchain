use serde_json::Result;

mod bank;
pub mod transfer;
mod vote;

pub enum Programs {
    Bank,
    Vote,
}

pub fn parse(x: &Programs, payload: &str) -> Result<()> {
    match x {
        Programs::Bank => bank::bank_program(payload),
        Programs::Vote => vote::vote_program(payload),
    }
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
    fn test_parse() {
        let payload = setup();
        let program = Programs::Bank;
        let _res = parse(&program, &payload).unwrap();
    }
}
