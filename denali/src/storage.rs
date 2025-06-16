use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct State(HashMap<String, String>);

impl State {
    pub fn new() -> Self {
        let mut inner = HashMap::default();
        // Write our program address for now
        inner.insert("fake_program".into(), "fake_program".into());
        State { 0: inner }
    }

    fn _exists(&self, address: String) -> bool {
        self.0.contains_key(&address)
    }

    pub fn get_value(&self, key: &str) {
        let _value = self.0.get(key);
    }

    fn _set_value() {}

    fn _new_key_value() {}
}
