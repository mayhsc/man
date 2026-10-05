use gb_core::cpu::Cpu;
use gb_core::bus::MemoryMap;

fn main() {
    let bus = MemoryMap::new();
    let mut cpu = Cpu::new(bus);

    cpu.step();
}
