use crate::bus::Bus;
use crate::bus::FlatBus;
use crate::cpu::Cpu;
use crate::cpu::registers::Registers;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CpuState {
    #[serde(flatten)]
    regs: Registers,
    ram: Vec<(u16, u8)>,
}

#[derive(Debug, Deserialize)]
struct TestCase {
    name: String,
    initial: CpuState,
    #[serde(rename = "final")]
    expected: CpuState,
}

fn bus_from_state(state: &CpuState) -> FlatBus {
    let mut bus = FlatBus::default();
    for &(addr, value) in &state.ram {
        bus.write(addr, value);
    }
    bus
}

fn run_test(tc: &TestCase, opcode: u8) {
    let bus = bus_from_state(&tc.initial);
    let mut cpu = Cpu::new(bus);

    cpu.regs = tc.initial.regs.clone();
    cpu.set_opcode(opcode);

    cpu.step();

    assert_state(&cpu, &tc.expected, &tc.name);
}

fn assert_state<B: Bus>(cpu: &Cpu<B>, expected: &CpuState, name: &str) {
    assert_eq!(cpu.regs, expected.regs, "{name}: registers differ");
    for &(addr, value) in &expected.ram {
        assert_eq!(
            cpu.bus.read(addr),
            value,
            "{name}: ram[{addr:#06x}] differs"
        );
    }
}

fn load(name: &str) -> Vec<TestCase> {
    let path = format!("{}/tests/sm83/{name}.json", env!("CARGO_MANIFEST_DIR"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{path}: {e}"))
}

pub fn run_file(name: &str) {
    for tc in load(name) {
        let opcode: u8 = parse_hex(name);
        run_test(&tc, opcode);
    }
}

fn parse_hex(s: &str) -> u8 {
    u8::from_str_radix(s, 16).unwrap()
}
