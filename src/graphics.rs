use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::memory::*;

pub const SCALE: u32 = 10;
pub const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
pub const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

// for 0xDxyn instruction
pub fn draw(mem: &mut MemoryComponents, x: usize, y: usize, spr_height: u16) {
    mem.v_reg[0xf] = 0;

    let x_coord = mem.v_reg[x] as u16;
    let y_coord = mem.v_reg[y] as u16;

    let mut flipped = false;

    for j in 0..spr_height {

        let address = mem.i_reg + j as u16;
        let pixels = mem.memory[address as usize];

        for i in 0..8 {
            if pixels & (0b1000_0000 >> i) != 0 {
                let actual_x = (x_coord + i) as usize % SCREEN_WIDTH; 
                let actual_y = (y_coord + j) as usize % SCREEN_HEIGHT; 

                flipped |= mem.display[actual_x][actual_y];
                mem.display[actual_x][actual_y] = true;
            }
        }
    }

    if flipped {
        mem.v_reg[0xf] = 1;
    } else {
        mem.v_reg[0xf] = 0;
    }
}

pub fn draw_on_gui_screen(mem: &MemoryComponents, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    for i in 0..SCREEN_WIDTH {
        for j in 0..SCREEN_HEIGHT {

            if mem.display[i][j] == true {
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
