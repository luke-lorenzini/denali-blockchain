use denali::types::Thing;
use vote::Votes;

fn main() {
    println!("Hello, vote");

    let vote = Votes::new(5);
    let payload = r#"
        {
            "candidate": 0
        }"#;
    let _res = vote.run(payload);
}
