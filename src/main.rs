extern crate chip8_emu;

use chip8_emu::Chip8;
use chip8_emu::utils::StrResult;
use std::env;
use std::path::PathBuf;

fn main() -> StrResult<()> {
    let args: Vec<String> = env::args().collect();
    let mut chip8 = Chip8::new();

    if args.len() != 2 {
        eprintln!("Usage: {} ROM_FILE", args[0]);
        return Err("Received invalid parameters");
    }

    chip8.load_rom(PathBuf::from(&args[1]))?;

    loop {
        if let Err(err) = chip8.exec_next_instruction() {
            return Err(err)
        }
    }

    Ok(())
}
