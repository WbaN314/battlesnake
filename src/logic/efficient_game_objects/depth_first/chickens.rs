use std::{collections::HashMap, ops::Deref, sync::Mutex};

pub struct Chickens(Mutex<HashMap<String, bool>>);

impl Chickens {
    pub fn new() -> Chickens {
        Chickens(Mutex::new(HashMap::new()))
    }
}

impl Deref for Chickens {
    type Target = Mutex<HashMap<String, bool>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
