/// An instance of a Chip 8 VM holding all necessary state,
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
/// * All fetches will build a proper opcode by joining PC & PC + 1
/// * The program counter must be incremented +2 after every fetch.
struct Chip8 {
    registers_v: [u8; 16],
    ir: u16,
    pc: u16,
    main_memory: [u8; Chip8::MAX_MEMORY_ADDRESS],
    stack: Stack,
    input: Input,
    display: Display,
}

/// Contains all operation implementations for the Chip 8 VM and 'magic numbers'
impl Chip8 {
    const INITIAL_MEMORY_ADDRESS: usize = 0x200;
    const MAX_MEMORY_ADDRESS: usize = 4096;

    /// Instantiates a new Chip 8 VM with proper initial values.
    /// # Initial values:
    /// * **Registers**: All set to `0x0`,
    /// * **Memory**: All addresses set to `0x0`,
    /// * **Stack**: All addresses set to `0x0` and the SP set to `0`,
    /// * **Input**: All 16 keys are set to `false` (non-pressed),
    /// * **Display**: All 32x64 pixels are set to `false`.
    fn new() -> Chip8 {
        Chip8 {
            registers_v: [0; 16],
            ir: 0,
            pc: Chip8::INITIAL_MEMORY_ADDRESS as u16,
            main_memory: [0; 4096],
            stack: Stack { stack_pointer: 0, stored_addresses: [0; 16] },
            input: Input { key_status: [false; 16] },
            display: Display { buffer: [[false; 64]; 32] },
        }
    }

    /// Loads the binary content of a ROM stored as a `Vec<u8>` inside a VM instance
    /// in the correct initial memory address.
    /// # Returns
    /// The amount of bytes that were loaded into `main_memory`.
    /// # Fails
    /// If the ROM is too big to be stored in memory.
    fn load_rom_content(&mut self, content: Vec<u8>) -> Result<usize, &str> {
        let content_size = content.len();
        let upper_memory_bound = Chip8::INITIAL_MEMORY_ADDRESS + content_size;
        if upper_memory_bound > Chip8::MAX_MEMORY_ADDRESS {
            return Err("ROM size exceeds memory capacity.");
        }

        self.main_memory[Chip8::INITIAL_MEMORY_ADDRESS..upper_memory_bound]
            .copy_from_slice(&content);

        return Ok(content_size);
    }
}

/// Holds the 16-level Chip 8 Stack and a single Stack Pointer.
/// # Stored Addresses:
/// Holds an ordered list of addresses which come from the Program Counter in a **Chip 8 VM**
/// # Stack Pointer:
/// Points to the next valid position in **stored_addresses** in which a memory address coming
/// from the PC can be stored with a **CALL** instruction.
struct Stack {
    stack_pointer: u8,
    stored_addresses: [u16; 16],
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
    /// Other possible implementations: 256 fixed size byte array.
    buffer: [[bool; 64]; 32]
}

struct Timers {
    // TODO: Currently not implemented!
}

use std::env;
use std::fs;
use std::process::exit;
use std::error::Error;

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