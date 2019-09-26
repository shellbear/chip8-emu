extern crate rand;

pub mod input;
pub mod memory;
pub mod registers;
pub mod screen;
pub mod stack;
pub mod timers;
pub mod utils;

use input::Input;
use memory::Memory;
use registers::Registers;
use screen::Screen;
use stack::Stack;
use timers::Timers;
use utils::StrResult;

use rand::{Rng, SeedableRng, XorShiftRng};
use std::path::PathBuf;
use std::fs::File;

const ENTRY_POINT: usize = 0x200;
const MEMORY_SIZE: usize = 0xfff;

pub struct Chip8<R: Rng> {
    pub memory: Memory,
    pub registers: Registers,
    pub stack: Stack,
    pub screen: Screen,
    pub timers: Timers,
    pub input: Input,
    rng: R
}

impl Chip8<XorShiftRng> {
    pub fn new() -> Self {
        Chip8::new_from_rng(SeedableRng::from_seed(rand::random()))
    }
}

impl<R: Rng> Chip8<R> {
    pub fn new_from_rng(r: R) -> Self {
        Self {
            memory: Memory::default(),
            registers: Registers::default(),
            stack: Stack::default(),
            screen: Screen::default(),
            timers: Timers::default(),
            input: Input::default(),
            rng: r
        }
    }

    pub fn load_rom(&mut self, file: PathBuf) -> StrResult<()> {
        let mut data = vec![];

        File::open(&file).and_then(|mut f| f.read_to_end(&mut data)).map_err(|_| "Could not read ROM")?;

        if data.len() > (MEMORY_SIZE - ENTRY_POINT) {
            return Err("ROM is too big");
        }

        self.memory.load(&data);

        Ok(())
    }

    pub fn exec_next_instruction(&mut self) -> StrResult<()> {
        let instr = (self.memory.read(self.registers.pc) as u16) << 8 |
            self.memory.read(self.registers.pc + 1) as u16;

        let op = [
            ((instr & 0xF000) >> 12) as u8,
            ((instr & 0x0F00) >> 8) as u8,
            ((instr & 0x00F0) >> 4) as u8,
            (instr & 0x000F) as u8
        ];

        self.registers.pc = self.registers.pc.wrapping_add(2);

        match (op[0], op[1], op[2], op[3]) {
            (0x0, 0x0, 0xE, 0x0) => {
                self.screen.clear();
                self.screen.should_draw = true;
            }, //	Clear the screen
            (0x0, 0x0, 0xE, 0xE) => {
                self.stack.sp -= 1;
                self.registers.pc = self.stack.ret_addresses[self.stack.sp as usize];
            }, //	Return from a subroutine
            (0x0, _, _, _) => println!("EXEC NNN"), //	        Execute machine language subroutine at address NNN
            (0x1, _, _, _) => self.registers.pc = instr & 0x0FFF, //	        Jump to address NNN
            (0x2, _, _, _) => {
                // Save the current address
                self.stack.ret_addresses[self.stack.sp as usize] = self.registers.pc;

                // Increment the stack pointer index
                self.stack.sp += 1;

                // Set the current program address to the provided address
                self.registers.pc = instr & 0x0FFF;
            }, //	        Execute subroutine starting at address NNN
            (0x3, _, _, _) => {
                if self.registers.v[op[1] as usize] == (instr & 0x00FF) as u8 {
                    self.registers.pc = self.registers.pc.wrapping_add(2);
                }
            }, //        Skip the following instruction if the value of register VX equals NN
            (0x4, _, _, _) => {
                if self.registers.v[op[1] as usize] != (instr & 0x00FF) as u8 {
                    self.registers.pc = self.registers.pc.wrapping_add(2);
                }
            }, //        Skip the following instruction if the value of register VX is not equal to NN
            (0x5, _, _, 0x0) => {
                if self.registers.v[op[1] as usize] == self.registers.v[op[2] as usize] {
                    self.registers.pc = self.registers.pc.wrapping_add(2);
                }
            }, //	    Skip the following instruction if the value of register VX is equal to the value of register VY
            (0x6, _, _, _) => self.registers.v[op[1] as usize] = (instr & 0x00FF) as u8, //        Store number NN in register VX
            (0x7, _, _, _) => self.registers.v[op[1] as usize] = self.registers.v[op[1] as usize].wrapping_add((instr & 0x00FF) as u8), //        Add the value NN to register VX
            (0x8, _, _, 0x0) => self.registers.v[op[1] as usize] = self.registers.v[op[2] as usize], //    Store the value of register VY in register VX
            (0x8, _, _, 0x1) => self.registers.v[op[1] as usize] |= self.registers.v[op[2] as usize], //    Set VX to VX OR VY
            (0x8, _, _, 0x2) => self.registers.v[op[1] as usize] &= self.registers.v[op[2] as usize], //    Set VX to VX AND VY
            (0x8, _, _, 0x3) => self.registers.v[op[1] as usize] ^= self.registers.v[op[2] as usize], //    Set VX to VX XOR VY
            (0x8, _, _, 0x4) => {
                let vx = self.registers.v[op[1] as usize];
                let vy = self.registers.v[op[2] as usize];
                let (result, overflow) = vx.overflowing_add(vy);

                self.registers.v[op[1] as usize] = result;
                self.registers.set_carry_flag(overflow);
            }, //    Add the value of register VY to register VX
                                             // Set VF to 01 if a carry occurs
                                             // Set VF to 00 if a carry does not occur
            (0x8, _, _, 0x5) => {
                let vx = self.registers.v[op[1] as usize];
                let vy = self.registers.v[op[2] as usize];
                let (result, overflow) = vx.overflowing_sub(vy);

                self.registers.v[op[1] as usize] = result;
                self.registers.set_carry_flag(overflow);
            }, //    Subtract the value of register VY from register VX
                                             // Set VF to 00 if a borrow occurs
                                             // Set VF to 01 if a borrow does not occur
            (0x8, _, _, 0x6) => {
                let vx = self.registers.v[op[1] as usize];

                self.registers.set_carry_flag(vx & 0x1 == 1);
                self.registers.v[op[1] as usize] = vx >> 1;
            }, //    Store the value of register VY shifted right one bit in register VX¹
                                             // Set register VF to the least significant bit prior to the shift
            (0x8, _, _, 0x7) => {
                let vx = self.registers.v[op[1] as usize];
                let vy = self.registers.v[op[2] as usize];
                let (result, overflow) = vy.overflowing_sub(vx);

                self.registers.v[op[1] as usize] = result;
                self.registers.set_carry_flag(overflow);
            }, //    Set register VX to the value of VY minus VX
                                             // Set VF to 00 if a borrow occurs
                                             // Set VF to 01 if a borrow does not occur
            (0x8, _, _, 0xE) => {
                let vx = self.registers.v[op[1] as usize];

                self.registers.set_carry_flag(((vx & 0x80) >> 7) == 1);
                self.registers.v[op[1] as usize] = vx << 1;
            }, //    Store the value of register VY shifted ri
                //    Store the value of register VY shifted left one bit in register VX¹
                                             // Set register VF to the most significant bit prior to the shift
            (0x9, _, _, 0x0) => {
                if self.registers.v[op[1] as usize] != self.registers.v[op[2] as usize] {
                    self.registers.pc = self.registers.pc.wrapping_add(2);
                }
            }, //    Skip the following instruction if the value of register VX is not equal to the value of register VY
            (0xA, _, _, _) => self.registers.i = instr & 0x0FFF, //        Store memory address NNN in register I
            (0xB, _, _, _) => self.registers.pc = (instr & 0x00FF).wrapping_add(self.registers.v[0x0] as u16), //        Jump to address NNN + V0
            (0xC, _, _, _) => {
                let random: u8 = self.rng.gen();

                self.registers.v[op[1] as usize] = random & ((instr & 0x00FF) as u8); //        Set VX to a random number with a mask of NN
            },
            (0xD, _, _, _) => {
                self.screen.should_draw = true;
            }, //        Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in I
                                           // Set VF to 01 if any set pixels are changed to unset, and 00 otherwise
            (0xE, _, 0x9, 0xE) => {
                if self.input.is_key_down(op[1]) {
                    self.registers.pc = self.registers.pc.wrapping_add(2);
                }
            }, //    Skip the following instruction if the key corresponding to the hex value currently stored in register VX is pressed
            (0xE, _, 0xA, 0x1) => {
                if !self.input.is_key_down(op[1]) {
                    self.registers.pc = self.registers.pc.wrapping_add(2);
                }
            }, //    Skip the following instruction if the key corresponding to the hex value currently stored in register VX is not pressed
            (0xF, _, 0x0, 0x7) => self.registers.v[op[1] as usize] = self.timers.delay_timer, //    Store the current value of the delay timer in register VX
            (0xF, _, 0x0, 0xA) => println!("READ INPUT"), //    Wait for a keypress and store the result in register VX
            (0xF, _, 0x1, 0x5) => self.timers.delay_timer = self.registers.v[op[1] as usize], //    Set the delay timer to the value of register VX
            (0xF, _, 0x1, 0x8) => self.timers.sound_timer = self.registers.v[op[1] as usize], //    Set the sound timer to the value of register VX
            (0xF, _, 0x1, 0xE) => self.registers.i = self.registers.i.wrapping_add(self.registers.v[op[1] as usize] as u16), //    Add the value stored in register VX to register I
            (0xF, _, 0x2, 0x9) => println!("I = SPRITE(VX)"), //    Set I to the memory address of the sprite data corresponding to the hexadecimal digit stored in register VX
            (0xF, _, 0x3, 0x3) => println!("STORE BINARY"), //    Store the binary-coded decimal equivalent of the value stored in register VX at addresses I, I + 1, and I + 2
            (0xF, _, 0x5, 0x5) => println!("STORE VALUES"), //    Store the values of registers V0 to VX inclusive in memory starting at address I
            _ => return Err("Received an invalid instruction.")
        };

        Ok(())
    }
}
