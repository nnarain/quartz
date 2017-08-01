extern crate libchip8;
extern crate sdl2;

use libchip8::*;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::cell::Cell;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let update_display = Cell::new(false);
    let mut vm = Chip8::new();

    vm.set_on_display_update(Box::new(
        || {
            update_display.set(true);
        }
    ));

    let filename = get_rom_filename(env::args()).unwrap_or_else(
        |e| {
            println!("{:?}", e);
            process::exit(1);
        }
    );

    let rom = load_rom_file(filename).unwrap_or_else(
        |e| {
            println!("{:?}", e);
            process::exit(1);
        }
    );

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Quartz", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    //let mut display = Surface::new(64, 32, PixelFormatEnum::RGB24).unwrap();
    let texture_creator = canvas.texture_creator();
    let mut display = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 64, 32).unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    vm.load_memory(rom);

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        if update_display.get() {
            canvas.clear();
            display.update(None, vm.get_display_memory(), 64 * 3).unwrap();
            canvas.copy(&display, None, Some(Rect::new(0,0, WINDOW_WIDTH, WINDOW_HEIGHT))).unwrap();
            canvas.present();

            update_display.set(false);
        }

        vm.step(512).unwrap();
    }
}

fn load_rom_file(rom_file: String) -> Result<Vec<u8>, Box<Error>> {
    let mut file = File::open(rom_file)?;

    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    Ok(buffer)
}

fn get_rom_filename(mut args: std::env::Args) -> Result<String, &'static str> {
    // skip first (exe name)
    args.next();

    let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("Error parsing command line options")
    };

    Ok(filename)
}
