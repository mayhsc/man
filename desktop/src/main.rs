use std::env;
use std::fs::File;
use std::io::Read;

use gb_core::bus::MemoryMap;
use gb_core::cpu::Cpu;
use sdl3::{
    VideoSubsystem, event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window,
};

const RESOLUTION_X: u32 = 160;
const RESOLUTION_Y: u32 = 144;
const SCALE: u32 = 10;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut canvas = create_canvas(&video_subsystem);

    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();

    let mut bus = MemoryMap::new();
    bus.load(&buffer);
    let mut cpu = Cpu::new(bus);

    'gameloop: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'gameloop;
                }
                _ => {}
            }
        }

        cpu.step();
        // draw_screen(cpu.);

        canvas.present();
    }
}

fn create_canvas(video_subsystem: &VideoSubsystem) -> Canvas<Window> {
    const WINDOW_WIDTH: u32 = (RESOLUTION_X as u32) * SCALE;
    const WINDOW_HEIGHT: u32 = (RESOLUTION_Y as u32) * SCALE;

    let window = video_subsystem
        .window("MAN", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let canvas = window.into_canvas();

    canvas
}

fn draw_screen(vram: &mut [u8]) {

}