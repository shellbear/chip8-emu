// More infos here: https://en.wikipedia.org/wiki/CHIP-8#Input

const KEYS_NUMBER: usize = 16;

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
