mod processor;
mod graphics;
mod memory;
mod font;

use sdl2::event::Event;
use std::fs::File;
use std::io::Read;
use std::env;

struct Chip8 {
    mem: memory::MemoryComponents,
    timer: processor::Timer,
}

fn main() { 

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} run /path/to/rom", args[0]);
        return;
    }
    let mut rom = File::open(&args[1])
        .expect("[err] unable to open file");

    let mut buffer = Vec::new(); 
    rom.read_to_end(&mut buffer).unwrap();

    let mut chip8_instance = Chip8 {
        mem: memory::MemoryComponents::new(),
        timer: processor::Timer::new(),
    };

    chip8_instance.mem.load_font();
    chip8_instance.mem.load_rom(&buffer); // rom content inside buffer will be moved to chip8's ram

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
                Event::Quit{..} => {break 'emulatorloop},
                _ => (),
            }
        }
        processor::cicle(&mut chip8_instance.mem);
        graphics::draw_on_gui_screen(&chip8_instance.mem, &mut canvas)
    }
}
