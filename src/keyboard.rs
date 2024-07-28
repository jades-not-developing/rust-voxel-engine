use std::collections::HashMap;

use glfw::Key;

#[derive(Default, Debug)]
pub struct Keyboard {
    map: HashMap<glfw::Key, bool>,
}

impl Keyboard {
    pub fn press(&mut self, key: Key) {
        self.map.insert(key, true);
    }

    pub fn release(&mut self, key: Key) {
        self.map.insert(key, false);
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        match self.map.get(&key) {
            Some(c) => *c,
            None => false,
        }
    }
}
