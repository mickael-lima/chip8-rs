use crate::memory::*;

use rand::Rng;

impl Chip8 {

    pub fn fetch_opcode_from(&mut self) -> u16 {  

        let most_sig_byte = self.memory[self.program_counter as usize] as u16;
        let last_sig_byte = self.memory[(self.program_counter + 1) as usize] as u16;

        let opcode: u16 = (most_sig_byte << 8) | last_sig_byte;
        self.program_counter += 2;

        opcode
    }

    pub fn execute(&mut self, opcode: u16) {
        let msb = (opcode & 0xf000) >> 12; // (0x??? & 0xf000 => 0x?000 >> 0x00?) 
        let sec_b = (opcode & 0x0f00) >> 8;
        let thd_b = (opcode & 0x00f0) >> 4; 
        let lsb = opcode & 0x000f;

        match (msb, sec_b, thd_b, lsb) {

            // do nothing (for sync and timing)
            (0, 0, 0, 0) => return,

            // clear screen
            (0, 0, 0xE, 0) => {
                self.display = [[false; SCREEN_HEIGHT]; SCREEN_WIDTH];    
            },

            // return from addr
            (0, 0, 0xE, 0xE) => {
                let address = self.stack_pop();
                self.program_counter = address as u16;
            },

            // jmp from current address (without storing it) to 0x0nnn
            (1, _, _, _) => {
                let new_address = opcode & 0x0FFF;
                self.program_counter = new_address;
            },

            // call subroutine at nnn
            (2, _, _, _) => {
                self.stack_push(self.program_counter);
                self.program_counter = opcode & 0x0FFF;
            },

            // skip to next instruction if V[x] = kk
            (3, _, _, _) => {
                if self.v_reg[sec_b as usize] == (opcode & 0x00FF) as u8 {
                    self.program_counter += 2;
                }
            },

            // same as the above, but check != instead of ==
            (4, _, _, _) => {
                if self.v_reg[sec_b as usize] as u16 != opcode & 0x00FF {
                    self.program_counter += 2;
                }
            },

            // skip to next instruction if v[x] != v[y]
            (5, _, _, 0) => {
                if self.v_reg[sec_b as usize] == self.v_reg[thd_b as usize] {
                    self.program_counter += 2;
                }
            },

            // set register V[x] to 0x00nn
            (6, _, _, _) => {
                let x = sec_b as usize; 
                self.v_reg[x] = (opcode & 0x00FF) as u8;
            },

            // V[x] = V[x] + kk
            (7, _, _, _) => {
                let x = sec_b as usize;
                let kk = (opcode & 0x00FF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(kk);
            },

            // set v[x] = v[y]
            (8, _, _, 0) => {
                self.v_reg[sec_b as usize] = self.v_reg[thd_b as usize];
            },

            // v[x] = OR between v[x] | v[y]
            (8, _, _, 1) => {
                self.v_reg[sec_b as usize] |= self.v_reg[thd_b as usize]; 
            },

            // v[x] = AND between v[x] & v[y]
            (8, _, _, 2) => {
                self.v_reg[sec_b as usize] &= self.v_reg[thd_b as usize]; 
            },

            // v[x] = XOR between v[x] ^ v[y]
            (8, _, _, 3) => {
                self.v_reg[sec_b as usize] ^= self.v_reg[thd_b as usize]; 
            },

            // v[x] = ADD operation between v[x], v[y], 
            (8, _, _, 4) => {
                let (new_vx, carry) = self.v_reg[sec_b as usize]
                    .overflowing_add(self.v_reg[thd_b as usize]); // in case of overflow, return a bool
                                                                   // too
                let new_vf = if carry {1} else {0};

                self.v_reg[sec_b as usize] = new_vx;
                self.v_reg[0xf] = new_vf;
            },

            // v[x] = SUB op between v[x], v[y]
            (8, _, _, 5) => {

                let (new_vx, borrow) = self.v_reg[sec_b as usize]
                    .overflowing_sub(self.v_reg[thd_b as usize]);

                let new_vf = if borrow {0} else {1};

                self.v_reg[sec_b as usize] = new_vx;
                self.v_reg[0xf] = new_vf;
            },

            // if lsb of v[x] is equal to 1, v[0xf] is set to 1 else 0, then v[x] = v[x]/2
            (8, _, _, 6) => {
                let lsb = self.v_reg[sec_b as usize] & 1;
                self.v_reg[sec_b as usize] >>= 1;
                self.v_reg[0xf] = lsb;
            }

            // v[x] = SUB op between v[y], v[x]
            (8, _, _, 7) => {

                let (new_vx, borrow) = self.v_reg[thd_b as usize]
                    .overflowing_sub(self.v_reg[sec_b as usize]);

                let new_vf = if borrow {0} else {1};

                self.v_reg[sec_b as usize] = new_vx;
                self.v_reg[0xf] = new_vf;
            },

            // same as 0x8xy6, but multiplies v[x] by 2
            (8, _, _, 0xE) => {
                let msb = (self.v_reg[sec_b as usize] >> 7) & 1;
                self.v_reg[sec_b as usize] <<= 1;
                self.v_reg[0xf] = msb;
            }

            // skip to next instruction if v[x] != v[y]
            (9, _, _, 0) => {
                if self.v_reg[sec_b as usize] != self.v_reg[thd_b as usize] {
                    self.program_counter += 2;
                }
            }

            // set i_reg to nnn 
            (0xA, _, _, _) => {
                let nnn = opcode & 0x0FFF;
                self.i_reg = nnn;
            },

            // jump to 0x0nnn + V[0]
            (0xB, _, _, _) => {
                let new_address = (opcode & 0x0FFF) + self.v_reg[0] as u16;
                self.program_counter = new_address;
            },

            // generates a random number between 0-255 and do AND operation between the value and 0x0kk
            // and store the result in v[x]
            (0xC, _, _, _) => {
                let mut rng = rand::thread_rng();
                self.v_reg[sec_b as usize] = rng.gen::<u8>() & (opcode & 0x00ff) as u8;    
            },

            // draw a n pixel's tall on screen at (x, y) (0xDXYN)
            (0xD, _, _, _) => {
                let x = sec_b;
                let y = thd_b;
                let n = lsb;
                self.draw(x as usize, y as usize, n);
            },

            // skip to next instruction if keyboard[x] is pressed 
            (0xE, _, 9, 0xE) => {

                let key = self.keyboard[self.v_reg[sec_b as usize] as usize];

                if key == true {
                    self.program_counter += 2;
                }
            },

            // same as the above, but check if it is not pressed
            (0xE, _, 0xA, 1) => {
                let key = self.keyboard[self.v_reg[sec_b as usize] as usize];

                if key == false {
                    self.program_counter += 2;
                }
            },

            // delay_timer value is set into v[x]
            (0xF, _, 0, 7) => { 
                self.v_reg[sec_b as usize] = self.delay_timer; 
            },

            // stop execution after a specific key is pressed, store in v[x]
            (0xF, _, 0, 0xA) => {

                let mut pressed = false;

                for i in 0..KEYBOARD_SIZE {
                    if self.keyboard[i] == true {
                        self.v_reg[sec_b as usize] = i as u8; 
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    self.program_counter -= 2;
                }
            },

            // set v[x] value at delay and sound_timer
            (0xF, _, 1, 5) => {
                self.delay_timer = self.v_reg[sec_b as usize];
            },

            (0xF, _, 1, 8) => {
                self.sound_timer = self.v_reg[sec_b as usize];
            },

            // store i + v[x] into i
            (0xF, _, 1, 0xE) => {
                let vx = self.v_reg[sec_b as usize] as u16; 
                self.i_reg = self.i_reg.wrapping_add(vx);
            },

            // set I to v[x], where v[x] in this scenario is a address that points to 
            // a sprite char of the font
            (0xF, _, 2, 9) => {
                // every char sprite of the font has a space of 5 bits between them
                self.i_reg = (self.v_reg[sec_b as usize] * 5) as u16;
            },

            (0xF, _, 3, 3) => {
                let number = self.v_reg[sec_b as usize] as f32;

                let hundred_digit = ((number / 100.0) % 10.0).floor() as u8;
                let ten_digit = ((number / 10.0) % 10.0).floor() as u8;
                let last_digit = (number % 10.0) as u8;

                self.memory[self.i_reg as usize] = hundred_digit;
                self.memory[(self.i_reg + 1) as usize] = ten_digit;
                self.memory[(self.i_reg + 2) as usize] = last_digit;
            },

            (0xF, _, 5, 5) => {
                for i in 0..=sec_b {
                    self.memory[(self.i_reg + i) as usize] = self.v_reg[i as usize]; 
                }
            },

            (0xF, _, 6, 5) => {
                for i in 0..=sec_b {
                    self.v_reg[i as usize] = self.memory[(self.i_reg + i) as usize]
                }
            }

            (_, _, _, _) => unimplemented!("unimplemented opcode: 0x{:x}", opcode),
        }
    }

    pub fn cicle(&mut self) {
        let actual_opcode = self.fetch_opcode_from();
        self.execute(actual_opcode);
    }
}
