mod chip_8;

use chip_8::Chip8;
use std::env;
use std::fs;
use std::process::exit;

const DEFAULT_CYCLE_DELAY: u64 = 10;

fn main() {
    let executable_name = env::args().nth(0).unwrap();
    let mut chip_8_vm = Chip8::new();

    let path = match env::args().nth(1) {
        None => {
            println!("ERROR: No ROM provided.");
            println!("Usage: {} (path-to-your-rom)", executable_name);
            exit(1);
        }
        Some(path) => path,
    };

    let rom_binary_content = match fs::read(&path) {
        Err(msg) => {
            println!("ERROR: Failed to open the ROM.");
            println!("Rust provided the next error message:\n>> {}", msg);
            exit(1);
        }
        Ok(content) => content,
    };

    let cycle_delay: u64 = match env::args().nth(2) {
        None => DEFAULT_CYCLE_DELAY,
        Some(delay) => match delay.parse::<u64>() {
            Ok(delay) => delay,
            Err(msg) => {
                println!("ERROR: {}", msg);
                exit(1);
            }
        },
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

    chip_8_vm.start(cycle_delay);
}
