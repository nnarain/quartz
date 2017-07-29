extern crate libchip8;
use libchip8::Chip8;

extern crate piston_window;
use piston_window::*;

use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn main() {
    // get rom filename from command line options
    // let rom_file = get_rom_filename(env::args()).unwrap_or_else(
    //     |err| {
    //         println!("{:?}", err);
    //         process::exit(1);
    //     }
    // );
    //
    // // read rom file
    // let rom = load_rom_file(rom_file).unwrap_or_else(
    //     |err| {
    //         println!("{:?}", err);
    //         process::exit(1);
    //     }
    // );

    // open a window
    let mut window: PistonWindow =
        WindowSettings::new("Quartz", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event,
            |context, graphics| {

            }
        );
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
