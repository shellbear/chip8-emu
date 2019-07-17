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

use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;

const ENTRY_POINT: usize = 0x200;
const MEMORY_SIZE: usize = 0xfff;

pub struct Chip8 {
    pub memory: Memory,
    pub registers: Registers,
    pub stack: Stack,
    pub screen: Screen,
    pub timers: Timers,
    pub input: Input
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: Memory::default(),
            registers: Registers::default(),
            stack: Stack::default(),
            screen: Screen::default(),
            timers: Timers::default(),
            input: Input::default()
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

    pub fn exec_next_instruction(&mut self) {
        let instr = (self.memory.read(self.registers.pc) as u16) << 8 |
            self.memory.read(self.registers.pc + 1) as u16;

        let op = [
            ((instr & 0xF000) >> 12) as u8,
            ((instr & 0x0F00) >> 8) as u8,
            ((instr & 0x00F0) >> 4) as u8,
            (instr & 0x000F) as u8
        ];

        self.registers.pc += 2;

        match (op[0], op[1], op[2], op[3]) {
            (0x0, 0x0, 0xE, 0x0) => println!("CLEAR SCREEN"), //	Clear the screen
            (0x0, 0x0, 0xE, 0xE) => println!("RETURN"), //	Return from a subroutine
            (0x0, _, _, _) => println!("EXEC NNN"), //	        Execute machine language subroutine at address NNN
            (0x1, _, _, _) => println!("JUMP NNN"), //	        Jump to address NNN
            (0x2, _, _, _) => println!("EXEC SUB NNN"), //	        Execute subroutine starting at address NNN
            (0x3, _, _, _) => println!("SKIP VX == NN"), //        Skip the following instruction if the value of register VX equals NN
            (0x4, _, _, _) => println!("SKIP VX != NN "), //        Skip the following instruction if the value of register VX is not equal to NN
            (0x5, _, _, 0x0) => println!("SKIP VX == VY"), //	    Skip the following instruction if the value of register VX is equal to the value of register VY
            (0x6, _, _, _) => println!("VX = NN"), //        Store number NN in register VX
            (0x7, _, _, _) => println!("VX += NNN"), //        Add the value NN to register VX
            (0x8, _, _, 0x0) => println!("VX = VY"), //    Store the value of register VY in register VX
            (0x8, _, _, 0x1) => println!("VX |= VY"), //    Set VX to VX OR VY
            (0x8, _, _, 0x2) => println!("VX |= VY"), //    Set VX to VX AND VY
            (0x8, _, _, 0x3) => println!("VX ^= VY"), //    Set VX to VX XOR VY
            (0x8, _, _, 0x4) => println!("VX += VY"), //    Add the value of register VY to register VX
                                             // Set VF to 01 if a carry occurs
                                             // Set VF to 00 if a carry does not occur
            (0x8, _, _, 0x5) => println!("VX -= VY"), //    Subtract the value of register VY from register VX
                                             // Set VF to 00 if a borrow occurs
                                             // Set VF to 01 if a borrow does not occur
            (0x8, _, _, 0x6) => println!("VX = VY >> 1"), //    Store the value of register VY shifted right one bit in register VX¹
                                             // Set register VF to the least significant bit prior to the shift
            (0x8, _, _, 0x7) => println!("VX = VY - VX"), //    Set register VX to the value of VY minus VX
                                             // Set VF to 00 if a borrow occurs
                                             // Set VF to 01 if a borrow does not occur
            (0x8, _, _, 0xE) => println!("VX = VY << 1"), //    Store the value of register VY shifted left one bit in register VX¹
                                             // Set register VF to the most significant bit prior to the shift
            (0x9, _, _, 0x0) => println!("SKIP VX != VY"), //    Skip the following instruction if the value of register VX is not equal to the value of register VY
            (0xA, _, _, _) => println!("I = NNN"), //        Store memory address NNN in register I
            (0xB, _, _, _) => println!("JMP NNN + V0"), //        Jump to address NNN + V0
            (0xC, _, _, _) => println!("VX = rand(NN)"), //        Set VX to a random number with a mask of NN
            (0xD, _, _, _) => println!("DRAW VX, VY, N, I"), //        Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in I
                                           // Set VF to 01 if any set pixels are changed to unset, and 00 otherwise
            (0xE, _, 0x9, 0xE) => println!("SKIP KEY PRESSED"), //    Skip the following instruction if the key corresponding to the hex value currently stored in register VX is pressed
            (0xE, _, 0xA, 0x1) => println!("SKIP KEY NOT PRESSED"), //    Skip the following instruction if the key corresponding to the hex value currently stored in register VX is not pressed
            (0xF, _, 0x0, 0x7) => println!("VX = DT"), //    Store the current value of the delay timer in register VX
            (0xF, _, 0x0, 0xA) => println!("VX = KEYPRESSED"), //    Wait for a keypress and store the result in register VX
            (0xF, _, 0x1, 0x5) => println!("dt = VX"), //    Set the delay timer to the value of register VX
            (0xF, _, 0x1, 0x8) => println!("st = VX"), //    Set the sound timer to the value of register VX
            (0xF, _, 0x1, 0xE) => println!("I += VX"), //    Add the value stored in register VX to register I
            (0xF, _, 0x2, 0x9) => println!("I = SPRITE(VX)"), //    Set I to the memory address of the sprite data corresponding to the hexadecimal digit stored in register VX
            (0xF, _, 0x3, 0x3) => println!(""), //    Store the binary-coded decimal equivalent of the value stored in register VX at addresses I, I + 1, and I + 2
            (0xF, _, 0x5, 0x5) => println!(""), //    Store the values of registers V0 to VX inclusive in memory starting at address I
            _ => eprintln!("Received an invalid instruction")
        };
    }
}
