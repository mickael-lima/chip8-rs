use crate::memory;
use crate::graphics;

use memory::*;
use rand::Rng;

pub struct Timer {
    delay_timer: u8,
    sound_timer: u8,
}

impl Timer {
    
    pub fn new() -> Self {
        Timer {
            delay_timer: 0,  
            sound_timer: 0,
        }
    }

    pub fn tick_timer(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -=1;
        }

        if self.sound_timer == 1 {
            // beep
        }

        self.sound_timer -= 1;
    } 
}

pub fn fetch_opcode_from(mem: &mut MemoryComponents) -> u16 {  

    let most_sig_byte = mem.memory[mem.program_counter as usize] as u16;
    let last_sig_byte = mem.memory[(mem.program_counter + 1) as usize] as u16;

    let opcode: u16 = (most_sig_byte << 8) | last_sig_byte;
    mem.program_counter += 2;
    
    opcode
}

pub fn execute(mem: &mut MemoryComponents, opcode: u16) {
    let msb = (opcode & 0xf000) >> 12; // (0x??? & 0xf000 => 0x?000 >> 0x00?) 
    let sec_b = (opcode & 0x0f00) >> 8;
    let thd_b = (opcode & 0x00f0) >> 4; 
    let lsb = opcode & 0x000f;

    match (msb, sec_b, thd_b, lsb) {
        
        // do nothing (for sync and timing)
        (0, 0, 0, 0) => return,
        
        // clear screen
        (0, 0, 0xE, 0) => {
            mem.display = [[false; SCREEN_HEIGHT]; SCREEN_WIDTH];    
        },

        // return from addr
        (0, 0, 0xE, 0xE) => {
            let address = mem.get_stack_top();
            mem.program_counter = address as u16;
            mem.stack_pointer -= 1;
        },

        // jmp from current address (without storing it) to 0x0nnn
        (1, _, _, _) => {
            let new_address = opcode & 0x0FFF;
            mem.program_counter = new_address as u16;
        },

        // call subroutine at nnn
        (2, _, _, _) => {
            mem.stack_pointer += 1;
            mem.stack_push(mem.program_counter);
            mem.program_counter = opcode & 0x0FFF;
        },

        // skip to next instruction if V[x] = kk
        (3, _, _, _) => {
            if mem.v_reg[sec_b as usize] as u16 == opcode & 0x00FF {
                mem.program_counter += 2;
            }
        },

        // same as the above, but check != instead of ==
        (4, _, _, _) => {
            if mem.v_reg[sec_b as usize] as u16 != opcode & 0x00FF {
                mem.program_counter += 2;
            }
        },

        // skip to next instruction if v[x] != v[y]
        (5, _, _, 0) => {
            if mem.v_reg[sec_b as usize] == mem.v_reg[thd_b as usize] {
                mem.program_counter += 2;
            }
        },

        // set register V[x] to 0x00nn
        (6, _, _, _) => {
            let x = sec_b as usize; 
            mem.v_reg[x] = (opcode & 0x00FF) as u8;
        },

        // V[x] = V[x] + kk
        (7, _, _, _) => {
            let x = sec_b as usize;
            mem.v_reg[x] += (opcode & 0x00FF) as u8;
        },

        // set v[x] = v[y]
        (8, _, _, 0) => {
            mem.v_reg[sec_b as usize] = mem.v_reg[thd_b as usize];
        },

        // v[x] = OR between v[x] | v[y]
        (8, _, _, 1) => {
            mem.v_reg[sec_b as usize] |= mem.v_reg[thd_b as usize]; 
        },

        // v[x] = AND between v[x] & v[y]
        (8, _, _, 2) => {
            mem.v_reg[sec_b as usize] &= mem.v_reg[thd_b as usize]; 
        },

        // v[x] = XOR between v[x] ^ v[y]
        (8, _, _, 3) => {
            mem.v_reg[sec_b as usize] ^= mem.v_reg[thd_b as usize]; 
        },

        // v[x] = ADD operation between v[x], v[y], 
        (8, _, _, 4) => {
            let sum_result = mem.v_reg[sec_b as usize] + mem.v_reg[thd_b as usize];

            // in case of a carry 
            if sum_result as u16 > 255 {
                mem.v_reg[0xf] = 1;
            } else {
                mem.v_reg[0xf] = 0;
            }

            mem.v_reg[sec_b as usize] = sum_result;
        },

        // v[x] = SUB op between v[x], v[y]
        (8, _, _, 5) => {
            if mem.v_reg[sec_b as usize] > mem.v_reg[thd_b as usize] {
                mem.v_reg[0xf] = 1;
            } else {
                mem.v_reg[0xf] = 0;
            }

            mem.v_reg[sec_b as usize] -= mem.v_reg[thd_b as usize];
        },

        // if lsb of v[x] is equal to 1, v[0xf] is set to 1 else 0, then v[x] = v[x]/2
        (8, _, _, 6) => {
            if (mem.v_reg[sec_b as usize] & 0x0f) == 1 {
                mem.v_reg[0xf] = 1;
            } else {
                mem.v_reg[0xf] = 0;
            }

            mem.v_reg[sec_b as usize] /= 2;
        }

        // v[x] = SUB op between v[y], v[x]
        (8, _, _, 7) => {
            if mem.v_reg[thd_b as usize] > mem.v_reg[sec_b as usize] {
                mem.v_reg[0xf] = 1;
            } else {
                mem.v_reg[0xf] = 0;
            }

            mem.v_reg[sec_b as usize] = mem.v_reg[thd_b as usize] - mem.v_reg[sec_b as usize];
        },

        // same as 0x8xy6, but multiplies v[x] by 2
        (8, _, _, 0xE) => {
            if (mem.v_reg[sec_b as usize] & 0x0f) == 1 {
                mem.v_reg[0xf] = 1;
            } else {
                mem.v_reg[0xf] = 0;
            }

            mem.v_reg[sec_b as usize] *= 2;
        }

        // skip to next instruction if v[x] != v[y]
        (9, _, _, 0) => {
            if mem.v_reg[sec_b as usize] != mem.v_reg[thd_b as usize] {
                mem.program_counter += 2;
            }
        }

        // set i_reg to nnn 
        (0xA, _, _, _) => {
            let nnn = opcode & 0x0FFF;
            mem.i_reg = nnn;
        },

        // jump to 0x0nnn + V[0]
        (0xB, _, _, _) => {
            let new_address = (opcode & 0x0FFF) + mem.v_reg[0] as u16;
            mem.program_counter = new_address;
        },

        // generates a random number between 0-255 and do AND operation between the value and 0x0kk
        // and store the result in v[x]
        (0xC, _, _, _) => {
            let mut rng = rand::thread_rng();
            mem.v_reg[sec_b as usize] = rng.gen::<u8>() & (opcode & 0x00ff) as u8;    
        },
        
        // draw pixel on screen (0xDXYN)
        (0xD, _, _, _) => {
            let x = sec_b;
            let y = thd_b;
            let n = lsb;
            graphics::draw(mem, x as usize, y as usize, n);
        },

        (_, _, _, _) => unimplemented!("unimplemented opcode: 0x{:x}", opcode),
    }
}

pub fn cicle(mem: &mut MemoryComponents) {
    let actual_opcode = fetch_opcode_from(mem);
    execute(mem, actual_opcode);
}
