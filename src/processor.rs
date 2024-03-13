use crate::memory;
use crate::graphics;

use memory::*;
use rand::Rng;

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
            let address = mem.stack_pop();
            mem.program_counter = address as u16;
        },

        // jmp from current address (without storing it) to 0x0nnn
        (1, _, _, _) => {
            let new_address = opcode & 0x0FFF;
            mem.program_counter = new_address;
        },

        // call subroutine at nnn
        (2, _, _, _) => {
            mem.stack_push(mem.program_counter);
            mem.program_counter = opcode & 0x0FFF;
        },

        // skip to next instruction if V[x] = kk
        (3, _, _, _) => {
            if mem.v_reg[sec_b as usize] == (opcode & 0x00FF) as u8 {
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
            let kk = (opcode & 0x00FF) as u8;
            mem.v_reg[x] = mem.v_reg[x].wrapping_add(kk);
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
            let (new_vx, carry) = mem.v_reg[sec_b as usize]
                .overflowing_add(mem.v_reg[thd_b as usize]); // in case of overflow, return a bool
                                                             // too
            let new_vf = if carry {1} else {0};

            mem.v_reg[sec_b as usize] = new_vx;
            mem.v_reg[0xf] = new_vf;
        },

        // v[x] = SUB op between v[x], v[y]
        (8, _, _, 5) => {
            
            let (new_vx, borrow) = mem.v_reg[sec_b as usize]
                .overflowing_sub(mem.v_reg[thd_b as usize]);

            let new_vf = if borrow {0} else {1};

            mem.v_reg[sec_b as usize] = new_vx;
            mem.v_reg[0xf] = new_vf;
        },

        // if lsb of v[x] is equal to 1, v[0xf] is set to 1 else 0, then v[x] = v[x]/2
        (8, _, _, 6) => {
            let lsb = mem.v_reg[sec_b as usize] & 1;
            mem.v_reg[sec_b as usize] >>= 1;
            mem.v_reg[0xf] = lsb;
        }

        // v[x] = SUB op between v[y], v[x]
        (8, _, _, 7) => {

            let (new_vx, borrow) = mem.v_reg[thd_b as usize]
                .overflowing_sub(mem.v_reg[sec_b as usize]);

            let new_vf = if borrow {0} else {1};

            mem.v_reg[sec_b as usize] = new_vx;
            mem.v_reg[0xf] = new_vf;
        },

        // same as 0x8xy6, but multiplies v[x] by 2
        (8, _, _, 0xE) => {
            let msb = (mem.v_reg[sec_b as usize] >> 7) & 1;
            mem.v_reg[sec_b as usize] <<= 1;
            mem.v_reg[0xf] = msb;
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
        
        // draw a n pixel's tall on screen at (x, y) (0xDXYN)
        (0xD, _, _, _) => {
            let x = sec_b;
            let y = thd_b;
            let n = lsb;
            graphics::draw(mem, x as usize, y as usize, n);
        },

        // skip to next instruction if keyboard[x] is pressed 
        (0xE, _, 9, 0xE) => {

            let key = mem.keyboard[mem.v_reg[sec_b as usize] as usize];

            if key == true {
                mem.program_counter += 2;
            }
        },

        // same as the above, but check if it is not pressed
        (0xE, _, 0xA, 1) => {
            let key = mem.keyboard[mem.v_reg[sec_b as usize] as usize];

            if key == false {
                mem.program_counter += 2;
            }
        },

        // delay_timer value is set into v[x]
        (0xF, _, 0, 7) => { 
            mem.v_reg[sec_b as usize] = mem.delay_timer; 
        },

        // stop execution after a specific key is pressed, store in v[x]
        (0xF, _, 0, 0xA) => {
            
            let mut pressed = false;

            for i in 0..memory::KEYBOARD_SIZE {
                if mem.keyboard[i] == true {
                    mem.v_reg[sec_b as usize] = i as u8; 
                    pressed = true;
                    break;
                }
            }

            if !pressed {
                mem.program_counter -= 2;
            }
        },

        // set v[x] value at delay and sound_timer
        (0xF, _, 1, 5) => {
            mem.delay_timer = mem.v_reg[sec_b as usize];
        },

        (0xF, _, 1, 8) => {
            mem.sound_timer = mem.v_reg[sec_b as usize];
        },
        
        // store i + v[x] into i
        (0xF, _, 1, 0xE) => {
            let vx = mem.v_reg[sec_b as usize] as u16; 
            mem.i_reg = mem.i_reg.wrapping_add(vx);
        },

        // set I to v[x], where v[x] in this scenario is a address that points to 
        // a sprite char of the font
        (0xF, _, 2, 9) => {
            // every char sprite of the font has a space of 5 bits between them
            mem.i_reg = (mem.v_reg[sec_b as usize] * 5) as u16;
        },

        (0xF, _, 3, 3) => {
            let number = mem.v_reg[sec_b as usize] as f32;

            let hundred_digit = ((number / 100.0) % 10.0).floor() as u8;
            let ten_digit = ((number / 10.0) % 10.0).floor() as u8;
            let last_digit = (number % 10.0) as u8;

            mem.memory[mem.i_reg as usize] = hundred_digit;
            mem.memory[(mem.i_reg + 1) as usize] = ten_digit;
            mem.memory[(mem.i_reg + 2) as usize] = last_digit;
        },

        (0xF, _, 5, 5) => {
            for i in 0..=sec_b {
                mem.memory[(mem.i_reg + i) as usize] = mem.v_reg[i as usize]; 
            }
        },

        (0xF, _, 6, 5) => {
            for i in 0..=sec_b {
                mem.v_reg[i as usize] = mem.memory[(mem.i_reg + i) as usize]
            }
        }

        (_, _, _, _) => unimplemented!("unimplemented opcode: 0x{:x}", opcode),
    }
}

pub fn cicle(mem: &mut MemoryComponents) {
    let actual_opcode = fetch_opcode_from(mem);
    execute(mem, actual_opcode);
}
