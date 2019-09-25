/// An instance of a Chip 8 VM holding all necessary state,
/// including registers, main memory, PC, etc.
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
    main_memory: [u8; Chip8::MAX_MEMORY_ADDRESS],
    regs: Registers,
    stack: Stack,
    input: Input,
    display: Display,
}

/// The Chip 8 uses 16 8-bit general purpose registers, labeled **v[0x0]** to **v[0xF]**
/// where **v[0xF]** holds information about the result of operations.
/// # Index register
/// 16-bit register that stores memory addresses for use in operations.
/// # Program counter:
/// 16-bit register that holds the address of the next to-be-executed operation.
struct Registers {
    v: [u8; 16],
    i: u16,
    pc: u16,
}

/// Holds the 16-level Chip 8 Stack and a single Stack Pointer.
/// # Stored Addresses:
/// Holds an ordered list of addresses which come from the Program Counter in a **Chip 8 VM**
/// # Stack Pointer:
/// Points to the next valid position in **stored_addresses** in which a memory address coming
/// from the PC can be stored with a **CALL** instruction.
struct Stack {
    pointer: u8,
    stored: [u16; 16],
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
            main_memory: [0; 4096],
            regs: Registers {
                v: [0; 16],
                i: 0,
                pc: Chip8::INITIAL_MEMORY_ADDRESS as u16,
            },
            stack: Stack {
                pointer: 0,
                stored: [0; 16],
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

    pub fn start(&mut self) {
        loop {
            self.cycle();
        }
    }

    /// Cycle emulation for the Chip 8 VM.
    /// During a `cycle` the VM will:
    /// - Fetch the next instruction and store as an opcode inside the `Chip8` struct
    /// - Update the `program counter` before any instruction execution takes place
    /// - Decode the `opcode` and execute it
    /// - Update both `Timers` (`delay` and `sound`) if needed
    fn cycle(&mut self) {
        // Fetch
        let opcode = self.fetch();

        // Update PC
        self.regs.pc += 2;

        // Decode and Execute
        self.execute(opcode);

        // TODO: Handle timers
    }

    fn fetch(&mut self) -> u16 {
        let lows = (self.main_memory[self.regs.pc as usize] as u16) << 8;
        let highs = self.main_memory[(self.regs.pc as usize) + 1] as u16;
        return lows | highs;
    }

    fn execute(&mut self, opcode: u16) {
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );

        let nnn = (opcode & 0x0FFF) as u16;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.cls(),
            (0x0, 0x0, 0xE, 0xE) => self.ret(),
            (0x1, _, _, _) => self.jp(nnn),
            (0x2, _, _, _) => self.call(nnn),
            (0x3, _, _, _) => self.se_vx_byte(x, kk),
            (0x4, _, _, _) => self.sne_vx_byte(x, kk),
            (0x5, _, _, 0x0) => self.se_vx_vy(x, y),
            (0x6, _, _, _) => self.ld_vx_value(x, kk),
            (0x7, _, _, _) => self.add_vx_byte(x, kk),
            (0x8, _, _, 0x0) => self.ld_vx_vy(x, y),
            (0x8, _, _, 0x1) => self.or_vx_vy(x, y),
            (0x8, _, _, 0x2) => self.and_vx_vy(x, y),
            (0x8, _, _, 0x3) => self.xor_vx_vy(x, y),
            (0x8, _, _, 0x4) => self.add_vx_vy(x, y),
            (0x8, _, _, 0x5) => self.sub_vx_vy(x, y),
            (0x8, _, _, 0x6) => self.shr_vx(x),
            (0x8, _, _, 0x7) => self.subn_vx_vy(x, y),
            (0x8, _, _, 0xE) => self.shl_vx(x),
            (0x9, _, _, 0x0) => self.sne_vx_vy(x, y),
            (0xA, _, _, _) => self.ld_i_addr(nnn),
            (0xB, _, _, _) => self.jp_v0_addr(nnn),
            (0xC, _, _, _) => self.rnd_vx_byte(x, kk),
            (0xD, _, _, _) => self.drw_vx_vy_n(x, y, n),
            (0xE, _, 0x9, 0xE) => self.skip_vx(x),
            (0xE, _, 0xA, 0x1) => self.skip_n_vx(x),
            (0xF, _, 0x0, 0x7) => self.ld_vx_dt(x),
            (0xF, _, 0x0, 0xA) => self.ld_vx_k(x),
            (0xF, _, 0x1, 0x5) => self.ld_dt_vx(x),
            (0xF, _, 0x1, 0x8) => self.ld_st_vx(x),
            (0xF, _, 0x1, 0xE) => self.add_i_vx(x),
            (0xF, _, 0x2, 0x9) => self.ld_f_vx(x),
            (0xF, _, 0x3, 0x3) => self.ld_b_vx(x),
            (0xF, _, 0x5, 0x5) => self.ld_i_vx(x),
            (0xF, _, 0x6, 0x5) => self.ld_vx_i(x),
            _ => {
                // Invalid operations are interpreted as NO OP
            }
        };
    }
}

/// Stack operations implementations
impl Stack {
    ///
    fn push(&mut self, value: u16) {
        self.stored[self.pointer as usize] = value;
        self.pointer += 1;
    }

    ///
    fn pop(&mut self) -> u16 {
        self.pointer -= 1;
        return self.stored[self.pointer as usize];
    }
}

/// Instruction implementation
impl Chip8 {
    /// **OP Code**: `00E0`
    ///
    /// Clear the diisplay
    fn cls(&mut self) {
        self.display.buffer = [[false; 64]; 32];
    }

    /// **OP Code**: `00EE`
    ///
    /// Return from a subroutine
    fn ret(&mut self) {
        self.regs.pc = self.stack.pop();
    }

    /// **OP Code**: `1nnn`
    ///
    /// Jump to address `nnn`
    fn jp(&mut self, nnn: u16) {
        self.regs.pc = nnn;
    }

    /// **OP Code**: `2nnn`
    ///
    /// Call subroutine at `nnn`
    fn call(&mut self, nnn: u16) {
        self.stack.push(self.regs.pc);
        self.regs.pc = nnn;
    }

    /// **OP Code**: `3xkk`
    ///
    /// Skip next instruction if `v[x]` == `kk`
    fn se_vx_byte(&mut self, x: usize, kk: u8) {
        if self.regs.v[x] == kk {
            self.regs.pc += 2;
        }
    }

    /// **OP Code**: `4xkk`
    ///
    /// Skip next instruction if `v[x] != kk`
    fn sne_vx_byte(&mut self, x: usize, kk: u8) {
        if self.regs.v[x] != kk {
            self.regs.pc += 2;
        }
    }

    /// **OP Code**: `5xy0`
    ///
    /// Skip next instruction if `v[x] == v[y]`
    fn se_vx_vy(&mut self, x: usize, y: usize) {
        if self.regs.v[x] == self.regs.v[y] {
            self.regs.pc += 2;
        }
    }

    /// **OP Code**: `6xkk`
    ///
    /// Set `v[x]` = `kk`
    fn ld_vx_value(&mut self, x: usize, kk: u8) {
        self.regs.v[x] = kk;
    }

    /// **OP Code**: `7xkk`
    ///
    /// Set `v[x] = v[x] + kk`
    fn add_vx_byte(&mut self, x: usize, kk: u8) {
        self.regs.v[x] += kk;
    }

    /// **OP Code**: `8xy0`
    ///
    /// Set `v[x] = v[y]`
    fn ld_vx_vy(&mut self, x: usize, y: usize) {
        self.regs.v[x] = self.regs.v[y];
    }

    /// **OP Code**: `8xy1`
    ///
    /// Set `v[x] = v[x] OR v[y]`
    fn or_vx_vy(&mut self, x: usize, y: usize) {
        self.regs.v[x] |= self.regs.v[y];
    }

    /// **OP Code**: `8xy2`
    ///
    /// Set `v[x] = v[x] AND v[y]`
    fn and_vx_vy(&mut self, x: usize, y: usize) {
        self.regs.v[x] &= self.regs.v[y];
    }

    /// **OP Code**: `8xy3`
    ///
    /// Set `v[x] = v[x] AND v[y]`
    fn xor_vx_vy(&mut self, x: usize, y: usize) {
        self.regs.v[x] ^= self.regs.v[y];
    }

    /// **OP Code**: `8xy4`
    ///
    /// Set `v[x] = v[x] + v[y]` and set `v[15] = carry`
    fn add_vx_vy(&mut self, x: usize, y: usize) {
        let sum = (self.regs.v[x] as u16) + (self.regs.v[y] as u16);

        self.regs.v[15] = if sum > 255 { 1 } else { 0 };
        self.regs.v[x] = (sum as u8) & 0xFF;
    }

    /// **OP Code**: `8xy5`
    ///
    /// Set `v[x] = v[x] - v[y]` and set `v[15] = not borrow`
    fn sub_vx_vy(&mut self, x: usize, y: usize) {
        self.regs.v[15] = if self.regs.v[x] > self.regs.v[y] {
            1
        } else {
            0
        };
        self.regs.v[x] -= self.regs.v[y];
    }

    /// **OP Code**: `8xy6`
    ///
    /// Set `v[x] = v[x] SHR 1`
    ///
    /// Set `v[15] = least-significant bit of v[x]`
    fn shr_vx(&mut self, x: usize) {
        self.regs.v[15] = self.regs.v[x] & 0x1;
        self.regs.v[x] >>= 1;
    }

    /// **OP Code**: `8xy7`
    ///
    /// Set `v[x] = v[y] - v[x]` and set `v[15] = not borrow`
    fn subn_vx_vy(&mut self, x: usize, y: usize) {
        self.regs.v[15] = if self.regs.v[y] > self.regs.v[x] {
            1
        } else {
            0
        };
        self.regs.v[x] = self.regs.v[y] - self.regs.v[x];
    }

    /// **OP Code**: `8xyE`
    ///
    /// Set `v[x] = v[x] SHL 1`
    ///
    /// Set `v[15] = most-significant bit of v[x]`
    fn shl_vx(&mut self, x: usize) {
        self.regs.v[15] = (self.regs.v[x] & 0x80) >> 7;
        self.regs.v[x] <<= 1;
    }

    /// **OP Code**: `9xy0`
    ///
    /// Skip next instruction if `v[x] != v[y]`
    fn sne_vx_vy(&mut self, x: usize, y: usize) {
        if self.regs.v[x] != self.regs.v[y] {
            self.regs.pc += 2;
        }
    }

    /// **OP Code**: `Annn`
    ///
    /// Set `IR = nnn`
    fn ld_i_addr(&mut self, nnn: u16) {
        self.regs.i = nnn;
    }

    /// **OP Code**: `Bnnn`
    ///
    /// Jump to address `v[0] + nnn`
    fn jp_v0_addr(&mut self, nnn: u16) {
        self.regs.pc = (self.regs.v[0] as u16) + nnn;
    }

    /// **OP Code**: `Cxkk`
    ///
    /// Set `v[x] = random byte AND kk`
    fn rnd_vx_byte(&mut self, x: usize, kk: u8) {
        let rand: u8 = 0xFF; // TODO: Add random generation

        self.regs.v[x] = rand & kk;
    }

    /// **OP Code**: `Dxyn`
    ///
    ///
    fn drw_vx_vy_n(&mut self, x: usize, y: usize, n: usize) {
        // TODO: Implement
    }

    /// **OP Code**: `Ex9E`
    ///
    /// Skip next instruction if the key with the value of `v[x]` is pressed
    fn skip_vx(&mut self, x: usize) {
        let key = self.regs.v[x] as usize;

        if self.input.key_status[key] {
            self.regs.pc += 2;
        }
    }

    /// **OP Code**: `ExA1`
    ///
    /// Skip next instruction if the key with the value of `v[x]` is pressed
    fn skip_n_vx(&mut self, x: usize) {
        let key = self.regs.v[x] as usize;

        if !self.input.key_status[key] {
            self.regs.pc += 2;
        }
    }

    /// **OP Code**: `Fx07`
    ///
    /// Set `v[x] = delay timer`
    fn ld_vx_dt(&mut self, x: usize) {
        let delay_timer: u8 = 0xFF; // TODO: Implement with real Delay Timer

        self.regs.v[x] = delay_timer;
    }

    /// **OP Code**: `Fx0A`
    ///
    /// Wait for a key press and store the value of the key in `v[x]`
    fn ld_vx_k(&mut self, x: usize) {
        match self
            .input
            .key_status
            .iter()
            .enumerate()
            .find(|(_, &is_active)| is_active)
        {
            Some((i, _)) => {
                self.regs.v[x] = i as u8;
            }
            None => {
                self.regs.pc -= 2;
            }
        };
    }

    /// **OP Code**: `Fx15`
    ///
    /// Set `delay timer = v[x]`
    fn ld_dt_vx(&mut self, x: usize) {
        // TODO: Implement
    }

    /// **OP Code**: `Fx18`
    ///
    /// Set `sound timer = v[x]`
    fn ld_st_vx(&mut self, x: usize) {
        // TODO: Implement
    }

    /// **OP Code**: `Fx1E`
    ///
    /// Set `IR = IR + v[x]`
    fn add_i_vx(&mut self, x: usize) {
        self.regs.i += self.regs.v[x] as u16;
    }

    /// **OP Code**: `Fx29`
    ///
    /// Set `IR = location of sprite for digit v[x]`
    fn ld_f_vx(&mut self, x: usize) {
        let digit = self.regs.v[x] as u16;

        self.regs.i = (Chip8::INITIAL_FONTS_MEMORY_ADDRESS as u16) + (5 * digit);
    }

    /// **OP Code**: `Fx33`
    ///
    /// Store BCD representation of `v[x]` in memory locations `[IR, IR + 1, IR + 2]`
    fn ld_b_vx(&mut self, x: usize) {
        let value = self.regs.v[x];

        self.main_memory[(self.regs.i as usize) + 2] = (value) % 10;
        self.main_memory[(self.regs.i as usize) + 1] = (value / 10) % 10;
        self.main_memory[self.regs.i as usize] = (value / 100) % 10;
    }

    /// **OP Code**: `Fx55`
    ///
    /// Store registers `v[0..X]` in memory starting at location `IR`
    fn ld_i_vx(&mut self, x: usize) {
        let memory_range = (self.regs.i as usize)..=(self.regs.i as usize) + x;
        self.main_memory[memory_range].copy_from_slice(&self.regs.v[0..=x]);
    }

    /// **OP Code**: `Fx65`
    ///
    /// Read registers `v[0..X]` from memory starting at location `IR`
    fn ld_vx_i(&mut self, x: usize) {
        let memory_range = (self.regs.i as usize)..=(self.regs.i as usize) + x;
        self.regs.v[0..=x].copy_from_slice(&self.main_memory[memory_range]);
    }
}
