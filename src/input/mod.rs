// More infos here: https://en.wikipedia.org/wiki/CHIP-8#Input

pub const KEYS_NUMBER: usize = 16;

pub struct Input {
    // CHIP-8 contains 16 keys, each key is represented by a boolean
    pub keys: [bool; KEYS_NUMBER]
}

impl Default for Input {
    fn default() -> Self {
        Self {
            keys: [false; KEYS_NUMBER]
        }
    }
}

impl Input {
    pub fn is_key_down(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn set_key_down(&mut self, key: u8, down: bool) {
        self.keys[key as usize] = down;
    }
}
