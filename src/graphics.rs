use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;

use crate::memory::*;

pub const SCALE: u32 = 10;
pub const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
pub const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

impl Chip8 {

    // for 0xDxyn instruction
    pub fn draw(&mut self, x: usize, y: usize, spr_height: u16) {
        self.v_reg[0xf] = 0;

        let x_coord = self.v_reg[x] as u16;
        let y_coord = self.v_reg[y] as u16;

        let mut flipped = false;

        for j in 0..spr_height {

            let address = self.i_reg + j as u16;
            let pixels = self.memory[address as usize];

            for i in 0..8 {
                if pixels & (0b1000_0000 >> i) != 0 {
                    let actual_x = (x_coord + i) as usize % SCREEN_WIDTH; 
                    let actual_y = (y_coord + j) as usize % SCREEN_HEIGHT; 

                    flipped |= self.display[actual_x][actual_y];
                    self.display[actual_x][actual_y] ^= true;
                }
            }
        }

        if flipped {
            self.v_reg[0xf] = 1;
        } else {
            self.v_reg[0xf] = 0;
        }
    }

    pub fn draw_on_gui_screen(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for i in 0..SCREEN_WIDTH {
            for j in 0..SCREEN_HEIGHT {

                if self.display[i][j] == true {
                    let rect = Rect::new(
                        (i as u32 * SCALE) as i32, 
                        (j as u32 * SCALE) as i32, 
                        SCALE, SCALE);

                    canvas.fill_rect(rect).unwrap();
                }
            }
        }

        canvas.present();
    }

    pub fn keyboardkey_to_number(&self, key: Keycode) -> Option<usize> {
        match key {
            Keycode::Num1 => Some(0x1),
            Keycode::Num2 => Some(0x2),
            Keycode::Num3 => Some(0x3),
            Keycode::Num4 => Some(0xC),
            Keycode::Q => Some(0x4),
            Keycode::W => Some(0x5),
            Keycode::E => Some(0x6),
            Keycode::R => Some(0xD),
            Keycode::A => Some(0x7),
            Keycode::S => Some(0x8),
            Keycode::D => Some(0x9),
            Keycode::F => Some(0xE),
            Keycode::Z => Some(0xA),
            Keycode::X => Some(0x0),
            Keycode::C => Some(0xB),
            Keycode::V => Some(0xF),

            _ => None,
        }
    }
}
