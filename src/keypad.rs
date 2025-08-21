use std::collections::HashMap;

pub struct Keypad {
    keys: HashMap<u8, bool>
}

impl Keypad {
    pub fn new() -> Self {
        let mut keys = HashMap::new();
        (0u8..16u8).for_each(|i| {keys.insert(i, false);});
        Self {
            keys
        }
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        self.keys
            .get(&key)
            .is_some_and(|pressed| *pressed)
    }

    pub fn press_key(&mut self, key: u8) {
        self.keys.insert(key, true);
    }

    pub fn release_key(&mut self, key: u8) {
        self.keys.insert(key, false);
    }
}