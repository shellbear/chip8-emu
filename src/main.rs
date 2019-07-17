extern crate chip8_emu;

use chip8_emu::Chip8;
use chip8_emu::utils::StrResult;
use std::path::PathBuf;

fn main() -> StrResult<()> {
    let mut chip8 = Chip8::new();

    let file = PathBuf::from("/home/shellbear/Documents/TETRIS.bin");

    chip8.load_rom(file)?;

    for _ in 1..10 {
        chip8.exec_next_instruction();
    }

    Ok(())
}
