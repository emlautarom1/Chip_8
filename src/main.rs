/// An instance of a Chip 8 VM holding all necesarry state,
/// including registers, main memory, PC, etc.
/// # Registers:
/// The Chip 8 uses 16 8-bit registers, labeled **v0** to **vF**
/// where the **VF** holds information about the result of operations.
/// # Index register
/// 16-bit register that stores memory addresses for use in operations.
/// # Program counter:
/// 16-bit register that holds the address of the next to-be-executed operation.
/// # Main memory:
/// The CHIP-8 has 4096 bytes of memory, meaning the address space
/// is from **0x000** to **0xFFF**.
/// ## Memory sections:
/// * **0x000** - **0x1FF**: Originally reserved for the Chip 8 interpreter.
/// * **0x050** - **0x0A0**: Storage for the 16 built-in characters.
/// * **0x200** - **0xFFF**: ROM instructions are loaded in this region and
/// all remaining space is free to be used as the developer sees fit.
///
/// ### Notes:
/// All opcodes are 2 bytes long, so:
/// * All fetchs will build a proper opcode by joining PC & PC + 1
/// * The program counter must be incremented +2 after every fetch.
///
/// Stack depth is 16 levels.
///
struct Chip8 {
    registers_v: [u8; 16],
    main_memory: [u8; 4096],
    stack: Stack,
    input: Input,
}

/// Holds the 16-level Chip 8 Stack and a single Stack Pointer.
/// # Stored Adresses:
/// Holds an ordered list of addresses wich come from the Program Counter in a **Chip 8 VM**
/// # Stack Pointer:
/// Points to the next valid position in **stored_adresses** in which a memory address coming
/// from the PC can be stored with a **CALL** instruction.
struct Stack {
    stack_pointer: u8,
    stored_adresses: [u16; 16],
}

/// Stores the current status of each 16 input keys, mapped from **0x0** to **0xF**
struct Input {
    key_status: [bool; 16]
}

/// Stores the display buffer of the Chip 8 VM.
/// The buffer is 64 pixels wide and 32 pixels high.
/// Only two values are accepted for each pixel: On or Off. We don't have color.
///
/// **Note:** All instruction that write outside the buffer valid range will wrap around.
struct Display {
    /// Tentative implementation as a boolean matrix.
    /// Access buffer values with: `buffer[row][col]`
    ///
    /// Other posible implementations: 256 fixed size byte array.
    buffer: [[bool; 64]; 32]
}

struct Timers {
    // TODO: Currently not implemented!
}

use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let rom_content: Vec<u8>;

    match env::args().nth(1) {
        Some(path) => {
            println!("Loading ROM {} ...", path);
            match fs::read(path) {
                Ok(binary_content) => {
                    println!("ROM loaded succesfully. {} bytes were read.", binary_content.len());
                    rom_content = binary_content;
                }
                Err(err) => {
                    println!("ERROR: Failed to open the ROM.");
                    println!("Rust provided the next error message:\n>> {}", err);
                    exit(1);
                }
            }
        }
        None => {
            println!("ERROR: No ROM provided.");
            println!("Usage: ./chip8.exe (path-to-your-rom)");
            exit(1);
        }
    };
}