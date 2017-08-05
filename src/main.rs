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
use std::collections::HashMap;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

fn main() {
    let update_display = Cell::new(false);

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

    // initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Quartz", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut display = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 64, 32).unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut key_map = HashMap::new();
    key_map.insert(Keycode::Q, 0x0);
    key_map.insert(Keycode::W, 0x1);
    key_map.insert(Keycode::E, 0x2);
    key_map.insert(Keycode::R, 0x3);
    key_map.insert(Keycode::T, 0x4);
    key_map.insert(Keycode::A, 0x5);
    key_map.insert(Keycode::S, 0x6);
    key_map.insert(Keycode::D, 0x7);
    key_map.insert(Keycode::F, 0x8);
    key_map.insert(Keycode::G, 0x9);
    key_map.insert(Keycode::Z, 0xA);
    key_map.insert(Keycode::X, 0xB);
    key_map.insert(Keycode::C, 0xC);
    key_map.insert(Keycode::V, 0xD);
    key_map.insert(Keycode::B, 0xE);
    key_map.insert(Keycode::Space, 0xF);

    let mut vm = Chip8::new(1.0/2.0e6);

    vm.set_on_display_update(Box::new(
        || {
            update_display.set(true);
        }
    ));

    vm.set_key_wait(Box::new(||{
        // let mut event_pump = sdl_context.event_pump().unwrap();
        //
        // loop {
        //     for event in event_pump.poll_iter() {
        //         match event {
        //             Event::KeyDown {keycode, ..} | Event::KeyUp {keycode, ..} => {
        //                 if let Some(keycode) = keycode {
        //                     if key_map.contains_key(&keycode) {
        //                         return key_map[&keycode];
        //                     }
        //                 }
        //             },
        //             _ => { continue }
        //         }
        //     }
        // }
        0
    }));

    vm.load_memory(rom);

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyUp {keycode, ..} => {
                    if let Some(k) = keycode {
                        if key_map.contains_key(&k) {
                            vm.key(key_map[&k], true);
                        }
                    }
                },
                Event::KeyDown {keycode, ..} =>  {
                    if let Some(k) = keycode {
                        if key_map.contains_key(&k) {
                            vm.key(key_map[&k], false);
                        }
                    }
                },
                _ => {}
            }
        }

        if update_display.get() {
            canvas.clear();
            display.update(None, vm.get_display_memory(), 64 * 3).unwrap();
            canvas.copy(&display, None, Some(Rect::new(0,0, WINDOW_WIDTH, WINDOW_HEIGHT))).unwrap();
            canvas.present();

            update_display.set(false);
        }

        vm.update(1).unwrap();
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
