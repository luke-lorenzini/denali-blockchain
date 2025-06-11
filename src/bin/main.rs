use denali::{Thing, vote::Votes};

fn main() {
    let vote = Votes::new(5);
    let payload = r#"
        {
            "candidate": 0
        }"#;
    let _res = vote.run(payload);
}
