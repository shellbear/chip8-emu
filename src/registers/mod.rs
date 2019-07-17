// More infos here: https://en.wikipedia.org/wiki/CHIP-8#Registers


use super::ENTRY_POINT;

pub const REGISTERS_NUMBER: usize = 16;

pub struct Registers {
    // Program counter
    pub pc: u16,

    // Index register
    pub i: u16,

    // CHIP-8 has 16 8-bit data registers named V0 to VF
    pub v: [u8; REGISTERS_NUMBER],
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            pc: ENTRY_POINT as u16,
            i: 0,
            v: [0; REGISTERS_NUMBER]
        }
    }
}

impl Registers {
    pub fn set_carry_flag(&mut self, carry: bool) {
        self.v[0xF] = carry as u8;
    }
}
