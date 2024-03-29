mod processor;
mod graphics;
mod memory;
mod font;

use sdl2::event::Event;
use std::env;

const TICK_PER_FRAME_REDRAW: u8 = 10;

fn main() { 

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} run /path/to/rom", args[0]);
        return;
    }

    let mut chip8_instance = memory::Chip8::new();

    chip8_instance.load_rom(&args[1]);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("CHIP-8", graphics::WINDOW_WIDTH, graphics::WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'emulatorloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit{..} => {
                    break 'emulatorloop
                },

                // detect key press and key release
                Event::KeyDown{keycode: Some(key), ..} => {
                    if let Some(k) = chip8_instance.keyboardkey_to_number(key) {
                        chip8_instance.keypress(k, true);
                    }
                },

                Event::KeyUp{keycode: Some(key), ..} => {
                    if let Some(k) = chip8_instance.keyboardkey_to_number(key) {
                        chip8_instance.keypress(k, false);
                    }
                },

                _ => (),
            }
        }

        // allows the emulator execute 10 operations before redrawing the frame
        // it'll make it run better
        for _ in 0..TICK_PER_FRAME_REDRAW {
            chip8_instance.cicle();
        }

        chip8_instance.tick_timers();
        chip8_instance.draw_on_gui_screen(&mut canvas)
    }
}
