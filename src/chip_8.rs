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
pub struct Chip8 {
    registers_v: [u8; 16],
    ir: u16,
    pc: u16,
    main_memory: [u8; Chip8::MAX_MEMORY_ADDRESS],
    stack: Stack,
    input: Input,
    display: Display,
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
    key_status: [bool; 16],
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
    buffer: [[bool; 64]; 32],
}

struct Timers {
    // TODO: Currently not implemented!
}

/// Contains all operation implementations for the Chip 8 VM and 'magic numbers'
impl Chip8 {
    const INITIAL_MEMORY_ADDRESS: usize = 0x200;
    const MAX_MEMORY_ADDRESS: usize = 4096;

    const INITIAL_FONTS_MEMORY_ADDRESS: usize = 0x50;
    const FONTS: [u8; 5 * 16] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];

    /// Instantiates a new Chip 8 VM with proper initial values.
    /// # Initial values:
    /// * **Registers**: All set to `0x0`,
    /// * **Program Counter**: Set to `INITIAL_MEMORY_ADDRESS`,
    /// * **Memory**: All addresses set to `0x0`,
    /// * **Stack**: All addresses set to `0x0` and the SP set to `0`,
    /// * **Input**: All 16 keys are set to `false` (non-pressed),
    /// * **Display**: All 32x64 pixels are set to `false`.
    /// # Panics
    /// If the VM can't load the initial fonts to memory. This should never happen, but if it does
    /// the host PC memory may be corrupted / damaged.
    pub fn new() -> Chip8 {
        let mut instance = Chip8 {
            registers_v: [0; 16],
            ir: 0,
            pc: Chip8::INITIAL_MEMORY_ADDRESS as u16,
            main_memory: [0; 4096],
            stack: Stack {
                stack_pointer: 0,
                stored_addresses: [0; 16],
            },
            input: Input {
                key_status: [false; 16],
            },
            display: Display {
                buffer: [[false; 64]; 32],
            },
        };

        if instance
            .load_to_memory(Chip8::INITIAL_FONTS_MEMORY_ADDRESS, &Chip8::FONTS)
            .is_err()
        {
            panic!("Failed to load initial fonts. VM could not be initialized.");
        }

        return instance;
    }

    /// Loads to the `main_memory` some binary content stored as `&Vec<u8>` in a specified `initial_address`
    /// # Returns
    /// The amount of bytes that were loaded into `main_memory`.
    /// # Fails
    /// If the `initial_address` exceeds the `MAX_MEMORY_ADDRESS` or if the content is too big
    /// to be stored in the `main_memory`
    fn load_to_memory(&mut self, initial_address: usize, content: &[u8]) -> Result<usize, &str> {
        if initial_address > Chip8::MAX_MEMORY_ADDRESS {
            return Err("Invalid initial address: exceeds MAX_MEMORY_ADDRESS");
        }

        let content_size = content.len();
        let end_address = initial_address + content_size;
        if end_address > Chip8::MAX_MEMORY_ADDRESS {
            return Err("Content can't be loaded outside memory bounds.");
        }

        self.main_memory[initial_address..end_address].copy_from_slice(content);

        return Ok(content_size);
    }

    /// Loads the binary content of a ROM stored as a `Vec<u8>` inside a VM instance
    /// in the correct initial memory address.
    /// # Returns
    /// The amount of bytes that were loaded into `main_memory`.
    /// # Fails
    /// If the ROM is too big to be stored in memory.
    pub fn load_rom_content(&mut self, content: Vec<u8>) -> Result<usize, &str> {
        return match self.load_to_memory(Chip8::INITIAL_MEMORY_ADDRESS, &content) {
            Ok(content_size) => Ok(content_size),
            Err(_) => Err("ROM size exceeds memory capacity."),
        };
    }
}
