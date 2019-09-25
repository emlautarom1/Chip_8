use super::Chip8;

impl Chip8 {
    /// **OP Code**: `00E0`
    ///
    /// Clear the diisplay
    pub fn cls(&mut self) {
        self.display.buffer = [[false; 64]; 32];
    }

    /// **OP Code**: `00EE`
    ///
    /// Return from a subroutine
    pub fn ret(&mut self) {
        self.regs.pc = self.stack.pop();
    }

    /// **OP Code**: `1nnn`
    ///
    /// Jump to address `nnn`
    pub fn jp(&mut self, nnn: u16) {
        self.regs.pc = nnn;
    }

    /// **OP Code**: `2nnn`
    ///
    /// Call subroutine at `nnn`
    pub fn call(&mut self, nnn: u16) {
        self.stack.push(self.regs.pc);
        self.regs.pc = nnn;
    }

    /// **OP Code**: `3xkk`
    ///
    /// Skip next instruction if `v[x]` == `kk`
    pub fn se_vx_byte(&mut self, x: usize, kk: u8) {
        if self.regs.v[x] == kk {
            self.regs.pc += 2;
        }
    }

    /// **OP Code**: `4xkk`
    ///
    /// Skip next instruction if `v[x] != kk`
    pub fn sne_vx_byte(&mut self, x: usize, kk: u8) {
        if self.regs.v[x] != kk {
            self.regs.pc += 2;
        }
    }

    /// **OP Code**: `5xy0`
    ///
    /// Skip next instruction if `v[x] == v[y]`
    pub fn se_vx_vy(&mut self, x: usize, y: usize) {
        if self.regs.v[x] == self.regs.v[y] {
            self.regs.pc += 2;
        }
    }

    /// **OP Code**: `6xkk`
    ///
    /// Set `v[x]` = `kk`
    pub fn ld_vx_value(&mut self, x: usize, kk: u8) {
        self.regs.v[x] = kk;
    }

    /// **OP Code**: `7xkk`
    ///
    /// Set `v[x] = v[x] + kk`
    pub fn add_vx_byte(&mut self, x: usize, kk: u8) {
        self.regs.v[x] += kk;
    }

    /// **OP Code**: `8xy0`
    ///
    /// Set `v[x] = v[y]`
    pub fn ld_vx_vy(&mut self, x: usize, y: usize) {
        self.regs.v[x] = self.regs.v[y];
    }

    /// **OP Code**: `8xy1`
    ///
    /// Set `v[x] = v[x] OR v[y]`
    pub fn or_vx_vy(&mut self, x: usize, y: usize) {
        self.regs.v[x] |= self.regs.v[y];
    }

    /// **OP Code**: `8xy2`
    ///
    /// Set `v[x] = v[x] AND v[y]`
    pub fn and_vx_vy(&mut self, x: usize, y: usize) {
        self.regs.v[x] &= self.regs.v[y];
    }

    /// **OP Code**: `8xy3`
    ///
    /// Set `v[x] = v[x] AND v[y]`
    pub fn xor_vx_vy(&mut self, x: usize, y: usize) {
        self.regs.v[x] ^= self.regs.v[y];
    }

    /// **OP Code**: `8xy4`
    ///
    /// Set `v[x] = v[x] + v[y]` and set `v[15] = carry`
    pub fn add_vx_vy(&mut self, x: usize, y: usize) {
        let sum = (self.regs.v[x] as u16) + (self.regs.v[y] as u16);

        self.regs.v[15] = if sum > 255 { 1 } else { 0 };
        self.regs.v[x] = (sum as u8) & 0xFF;
    }

    /// **OP Code**: `8xy5`
    ///
    /// Set `v[x] = v[x] - v[y]` and set `v[15] = not borrow`
    pub fn sub_vx_vy(&mut self, x: usize, y: usize) {
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
    pub fn shr_vx(&mut self, x: usize) {
        self.regs.v[15] = self.regs.v[x] & 0x1;
        self.regs.v[x] >>= 1;
    }

    /// **OP Code**: `8xy7`
    ///
    /// Set `v[x] = v[y] - v[x]` and set `v[15] = not borrow`
    pub fn subn_vx_vy(&mut self, x: usize, y: usize) {
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
    pub fn shl_vx(&mut self, x: usize) {
        self.regs.v[15] = (self.regs.v[x] & 0x80) >> 7;
        self.regs.v[x] <<= 1;
    }

    /// **OP Code**: `9xy0`
    ///
    /// Skip next instruction if `v[x] != v[y]`
    pub fn sne_vx_vy(&mut self, x: usize, y: usize) {
        if self.regs.v[x] != self.regs.v[y] {
            self.regs.pc += 2;
        }
    }

    /// **OP Code**: `Annn`
    ///
    /// Set `IR = nnn`
    pub fn ld_i_addr(&mut self, nnn: u16) {
        self.regs.i = nnn;
    }

    /// **OP Code**: `Bnnn`
    ///
    /// Jump to address `v[0] + nnn`
    pub fn jp_v0_addr(&mut self, nnn: u16) {
        self.regs.pc = (self.regs.v[0] as u16) + nnn;
    }

    /// **OP Code**: `Cxkk`
    ///
    /// Set `v[x] = random byte AND kk`
    pub fn rnd_vx_byte(&mut self, x: usize, kk: u8) {
        let rand: u8 = rand::random();

        self.regs.v[x] = rand & kk;
    }

    /// **OP Code**: `Dxyn`
    ///
    ///
    pub fn drw_vx_vy_n(&mut self, x: usize, y: usize, n: usize) {
        // TODO: Implement
    }

    /// **OP Code**: `Ex9E`
    ///
    /// Skip next instruction if the key with the value of `v[x]` is pressed
    pub fn skip_vx(&mut self, x: usize) {
        let key = self.regs.v[x] as usize;

        if self.input.key_status[key] {
            self.regs.pc += 2;
        }
    }

    /// **OP Code**: `ExA1`
    ///
    /// Skip next instruction if the key with the value of `v[x]` is pressed
    pub fn skip_n_vx(&mut self, x: usize) {
        let key = self.regs.v[x] as usize;

        if !self.input.key_status[key] {
            self.regs.pc += 2;
        }
    }

    /// **OP Code**: `Fx07`
    ///
    /// Set `v[x] = delay timer`
    pub fn ld_vx_dt(&mut self, x: usize) {
        let delay_timer: u8 = 0xFF; // TODO: Implement with real Delay Timer

        self.regs.v[x] = delay_timer;
    }

    /// **OP Code**: `Fx0A`
    ///
    /// Wait for a key press and store the value of the key in `v[x]`
    pub fn ld_vx_k(&mut self, x: usize) {
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
    pub fn ld_dt_vx(&mut self, x: usize) {
        // TODO: Implement
    }

    /// **OP Code**: `Fx18`
    ///
    /// Set `sound timer = v[x]`
    pub fn ld_st_vx(&mut self, x: usize) {
        // TODO: Implement
    }

    /// **OP Code**: `Fx1E`
    ///
    /// Set `IR = IR + v[x]`
    pub fn add_i_vx(&mut self, x: usize) {
        self.regs.i += self.regs.v[x] as u16;
    }

    /// **OP Code**: `Fx29`
    ///
    /// Set `IR = location of sprite for digit v[x]`
    pub fn ld_f_vx(&mut self, x: usize) {
        let digit = self.regs.v[x] as u16;

        self.regs.i = (Chip8::INITIAL_FONTS_MEMORY_ADDRESS as u16) + (5 * digit);
    }

    /// **OP Code**: `Fx33`
    ///
    /// Store BCD representation of `v[x]` in memory locations `[IR, IR + 1, IR + 2]`
    pub fn ld_b_vx(&mut self, x: usize) {
        let value = self.regs.v[x];

        self.main_memory[(self.regs.i as usize) + 2] = (value) % 10;
        self.main_memory[(self.regs.i as usize) + 1] = (value / 10) % 10;
        self.main_memory[self.regs.i as usize] = (value / 100) % 10;
    }

    /// **OP Code**: `Fx55`
    ///
    /// Store registers `v[0..X]` in memory starting at location `IR`
    pub fn ld_i_vx(&mut self, x: usize) {
        let memory_range = (self.regs.i as usize)..=(self.regs.i as usize) + x;
        self.main_memory[memory_range].copy_from_slice(&self.regs.v[0..=x]);
    }

    /// **OP Code**: `Fx65`
    ///
    /// Read registers `v[0..X]` from memory starting at location `IR`
    pub fn ld_vx_i(&mut self, x: usize) {
        let memory_range = (self.regs.i as usize)..=(self.regs.i as usize) + x;
        self.regs.v[0..=x].copy_from_slice(&self.main_memory[memory_range]);
    }
}
