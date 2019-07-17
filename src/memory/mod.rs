// More infos here: https://en.wikipedia.org/wiki/CHIP-8#Memory

const MEMORY_SIZE: usize = 0xfff;

pub struct Memory {
    // The ram is simply an array of 4096 u8
    pub ram: [u8; MEMORY_SIZE]
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            ram: [0; MEMORY_SIZE]
        }
    }
}

impl Memory {
    pub fn read(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.ram[addr as usize] = val;
    }
}
