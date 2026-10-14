use std::cell::RefCell;

use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::cpu::registers::Registers;
use serde::Deserialize;

pub(crate) enum TestType {
    Opcode,
    Cycle,
}

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
    cycles: Vec<Option<(u16, u8, String)>>,
}

pub struct FlatBus {
    pub memory: [u8; 0x10000],
}

impl Default for FlatBus {
    fn default() -> Self {
        Self { memory: [0; 65536] }
    }
}

impl Bus for FlatBus {
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
}

struct TracingBus {
    inner: FlatBus,
    log: RefCell<Vec<(u16, u8, &'static str)>>,
}

impl Default for TracingBus {
    fn default() -> Self {
        Self {
            inner: FlatBus::default(),
            log: RefCell::new(vec![]),
        }
    }
}

impl Bus for TracingBus {
    fn read(&self, addr: u16) -> u8 {
        let v = self.inner.read(addr);
        self.log.borrow_mut().push((addr, v, "read"));
        v
    }
    fn write(&mut self, addr: u16, value: u8) {
        self.inner.write(addr, value);
        self.log.borrow_mut().push((addr, value, "write"));
    }

    fn peek(&self, addr: u16) -> u8 {
        self.inner.peek(addr)
    }
}

fn flat_bus_from_state(state: &CpuState) -> FlatBus {
    let mut bus = FlatBus::default();
    for &(addr, value) in &state.ram {
        bus.write(addr, value);
    }
    bus
}

fn tracing_bus_from_state(state: &CpuState) -> TracingBus {
    let mut bus = TracingBus::default();
    for &(addr, value) in &state.ram {
        bus.write(addr, value);
    }
    bus
}

fn run_opcode_test(tc: &TestCase, opcode: u8) {
    let bus = flat_bus_from_state(&tc.initial);
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

fn run_cycle_test(tc: &TestCase, opcode: u8) {
    let bus = tracing_bus_from_state(&tc.initial);
    let mut cpu = Cpu::new(bus);

    cpu.regs = tc.initial.regs.clone();
    cpu.set_opcode(opcode);
    cpu.bus.log.borrow_mut().clear();

    let m_cycles = cpu.step();

    assert_eq!(m_cycles as usize, tc.cycles.len(), "{}: m_cycles", tc.name);

    let expected: Vec<_> = tc.cycles.iter().flatten().collect();
    let actual = cpu.bus.log.borrow();

    assert_eq!(
        actual.len(),
        expected.len(),
        "{}: bus access count (actual {:?}, expected {:?})",
        tc.name,
        *actual,
        expected
    );

    for (i, (a, e)) in actual.iter().zip(&expected).enumerate() {
        assert_eq!(
            (a.0, a.1, a.2),
            (e.0, e.1, e.2.as_str()),
            "{}: bus access #{i}",
            tc.name
        );
    }
}

fn load(name: &str) -> Vec<TestCase> {
    let path = format!("{}/tests/sm83/{name}.json", env!("CARGO_MANIFEST_DIR"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{path}: {e}"))
}

pub fn run_file(name: &str, test_type: TestType) {
    for tc in load(name) {
        let opcode: u8 = parse_hex(name);
        match test_type {
            TestType::Opcode => {
                run_opcode_test(&tc, opcode);
            }
            TestType::Cycle => {
                run_cycle_test(&tc, opcode);
            }
        }
    }
}

fn parse_hex(s: &str) -> u8 {
    u8::from_str_radix(s, 16).unwrap()
}
