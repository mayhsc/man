use std::env;
use std::fs::File;
use std::io::Read;

use gb_core::bus::MemoryMap;
use gb_core::cpu::Cpu;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();


    let mut bus = MemoryMap::new();
    bus.load(&buffer);
    let mut cpu = Cpu::new(bus);

    loop {
        cpu.step();
    }
}
