extern crate libchip8;
use libchip8::*;
extern crate piston_window;
extern crate image as im;

use piston_window::*;

use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::cell::Cell;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

fn main() {
    let update_display = Cell::new(false);
    let mut vm = VirtualMachine::new();

    //get rom filename from command line options
    let rom_file = get_rom_filename(env::args()).unwrap_or_else(
        |err| {
            println!("{:?}", err);
            process::exit(1);
        }
    );

    // read rom file
    let rom = load_rom_file(rom_file).unwrap_or_else(
        |err| {
            println!("{:?}", err);
            process::exit(1);
        }
    );

    vm.load_memory(rom);

    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("Quartz", (WINDOW_WIDTH, WINDOW_HEIGHT))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut framebuffer = im::ImageBuffer::new(64, 32);
    let mut display = Texture::from_image(
            &mut window.factory,
            &framebuffer,
            &TextureSettings::new()
        ).unwrap();

    vm.set_on_display_update(Box::new(||{
        update_display.set(true);
    }));

    while let Some(e) = window.next() {
        if update_display.get() {
            for x in 0..64u32 {
                for y in 0..32u32 {
                    let (r, g, b) = vm.get_pixel(x as usize, y as usize);
                    framebuffer.put_pixel(x, y, im::Rgba([r, g, b, 255]));
                }
            }

            display.update(&mut window.encoder, &framebuffer).unwrap();
            update_display.set(false);
        }

        if let Some(_) = e.render_args() {
        //    display.update(&mut window.encoder, &framebuffer).unwrap();
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                image(&display, c.transform.scale(5.0, 5.0), g);
            });
        }

        vm.step(512).unwrap_or_else(
            |e| {
                println!("{:?}", e);
                process::exit(1);
            }
        )
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
