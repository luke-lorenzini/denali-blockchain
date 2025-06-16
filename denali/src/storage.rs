use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Debug)]
pub struct State(HashMap<String, String>);

impl State {
    pub fn new() -> Self {
        let mut inner = HashMap::new();
        // Write our program address for now
        inner.insert("fake_program".into(), "fake_program".into());
        State(inner)
    }

    fn _exists(&self, address: String) -> bool {
        self.0.contains_key(&address)
    }

    pub fn get_value(&self, key: &str) {
        let _value = self.0.get(key);
    }

    pub fn set_value(&mut self, key: &str, value: &str) {
        if self.0.contains_key(key) {
            let val = self.0.get_mut(key).expect("Already checked");
            *val = String::from_str(value).unwrap();
        } else {
            self.0.insert(key.into(), value.into());
        }
    }

    pub fn new_key_value() {}
}
