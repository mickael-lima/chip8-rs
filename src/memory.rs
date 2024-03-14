use crate::font;

use std::fs::File;
use std::io::Read;

const STACK_SIZE: usize = 16;
const RAM_SIZE: usize = 4096;
const REG_SIZE: usize = 16;

pub const KEYBOARD_SIZE: usize = 16;
pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_WIDTH: usize = 64;

pub struct Chip8 {
    pub display: [[bool; SCREEN_HEIGHT]; SCREEN_WIDTH],
    pub memory: [u8; RAM_SIZE],
    pub keyboard: [bool; KEYBOARD_SIZE],
    pub stack: [u16; STACK_SIZE],
    pub v_reg: [u8; REG_SIZE],
    pub i_reg: u16,
    pub program_counter: u16,
    pub stack_pointer: usize,
    pub delay_timer: u8,
    pub sound_timer: u8,
}

// general functions
impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            display: [[false; SCREEN_HEIGHT]; SCREEN_WIDTH],
            memory: [0; RAM_SIZE],
            keyboard: [false; KEYBOARD_SIZE],
            stack: [0; STACK_SIZE],
            v_reg: [0; REG_SIZE],
            i_reg: 0,
            program_counter: 0x200,
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {self.delay_timer -= 1;}

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
            // TODO: beep sound
            }

            self.sound_timer -= 1;
        }
    }

    pub fn keypress(&mut self, keycode: usize, status: bool) {
        self.keyboard[keycode] = status; 
    }
}

// file & memory management
impl Chip8 {

    pub fn load_font(&mut self) -> [u8; RAM_SIZE]{
        for i in 0..font::FONT_SET.len() {
            self.memory[i] = font::FONT_SET[i];
        }

        self.memory
    }

    pub fn load_rom(&mut self, filename: &String) {
        const MAX_SIZE_FOR_ROM_DATA: u16 = (RAM_SIZE - 0x200) as u16;

        let mut rom_file = File::open(filename).unwrap();
        self.load_font();

        let mut rom_raw_data = Vec::new();
        rom_file.read_to_end(&mut rom_raw_data).unwrap();

        if rom_raw_data.len() as u16 > MAX_SIZE_FOR_ROM_DATA {
            panic!("File {} is too big for be loaded into the emulator, exiting...", filename);
        }

        let last_addr = 0x200 + rom_raw_data.len() as usize;
        self.memory[0x200..last_addr].copy_from_slice(&rom_raw_data);
    }
}

// stack management
impl Chip8 {

    // note: there's no need to check for overflow and underflow
    // condition because it is handled in opcode's section
    pub fn stack_push(&mut self, value: u16) {
        self.stack[self.stack_pointer] = value;
        self.stack_pointer += 1;
    }

    pub fn stack_pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }
}
