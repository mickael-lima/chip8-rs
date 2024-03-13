use crate::font;

const STACK_SIZE: usize = 16;
const RAM_SIZE: usize = 4096;
const REG_SIZE: usize = 16;

pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_WIDTH: usize = 64;

pub struct MemoryComponents {
    pub display: [[bool; SCREEN_HEIGHT]; SCREEN_WIDTH],
    pub memory: [u8; RAM_SIZE],
    pub stack: [u16; STACK_SIZE],
    pub v_reg: [u8; REG_SIZE],
    pub i_reg: u16,
    pub program_counter: u16,
    pub stack_pointer: usize,
}

impl MemoryComponents {

    pub fn load_font(&mut self) -> [u8; RAM_SIZE]{
        for i in 0..font::FONT_SET.len() {
            self.memory[i] = font::FONT_SET[i];
        }

        self.memory
    }

    pub fn stack_push(&mut self, value: u16) {
        const OVERFLOW_INDEX: usize = STACK_SIZE + 1;

        match self.stack_pointer {
            OVERFLOW_INDEX => self.stack_pointer = 0,
            _ => {
                self.stack[self.stack_pointer] = value;
                self.stack_pointer += 1;
            },
        }
    }

    pub fn stack_pop(&mut self) {

        if self.stack_pointer == 0 {
            println!("[wrn] stack_pop() tried to pop at index 0");
            println!("[wrn] stack_pointer will reset to 0");

            self.stack_pointer = 1;
        }

        self.stack_pointer -= 1;
        self.stack[self.stack_pointer] = 0;
    }

    pub fn get_stack_top(&self) -> usize {

        let mut top_index = 0;

        for i in 0..STACK_SIZE {
            if self.stack[i] == 0 {
                top_index = i;
                break;
            }
        }

        top_index - 1
    }

    pub fn new() -> MemoryComponents {
        MemoryComponents {
            display: [[false; SCREEN_HEIGHT]; SCREEN_WIDTH],
            memory: [0; RAM_SIZE],
            stack: [0; STACK_SIZE],
            v_reg: [0; REG_SIZE],
            i_reg: 0,
            program_counter: 0x200,
            stack_pointer: 0,
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        let last_addr = 0x200 + data.len() as usize;
        self.memory[0x200..last_addr].copy_from_slice(data);
    }
}
