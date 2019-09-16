mod chip_8;

use chip_8::*;
use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let mut chip_8_vm = Chip8::new();

    let path = match env::args().nth(1) {
        None => {
            println!("ERROR: No ROM provided.");
            println!("Usage: ./chip8.exe (path-to-your-rom)");
            exit(1);
        }
        Some(path) => path
    };

    let rom_binary_content = match fs::read(&path) {
        Err(msg) => {
            println!("ERROR: Failed to open the ROM.");
            println!("Rust provided the next error message:\n>> {}", msg);
            exit(1);
        }
        Ok(content) => content,
    };

    println!("Loading ROM {} ...", &path);
    match chip_8_vm.load_rom_content(rom_binary_content) {
        Err(msg) => {
            println!("ERROR: {}", msg);
            exit(1);
        }
        Ok(total_read) => {
            println!("ROM loaded successfully. {} bytes were read.", total_read);
        }
    }
}