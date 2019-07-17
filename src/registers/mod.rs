const REGISTERS_NUMBER: usize = 16;

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
            pc: 0,
            i: 0,
            v: [0; REGISTERS_NUMBER]
        }
    }
}

impl Registers {
    pub fn get_next_pc(&mut self) -> u16 {
        // Prevent overflow by resetting counter
        self.pc = if self.pc == 0xfe {
            0
        } else {
            self.pc + 1
        };

        self.pc
    }
}
