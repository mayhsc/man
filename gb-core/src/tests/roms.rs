use super::harness::rom_harness::run_file;

macro_rules! rom_tests {
    ($($fn_name:ident => $file:literal),* $(,)?) => {
        $(
            #[test]
            fn $fn_name() {
                run_file($file);
            }
        )*
    };
}

rom_tests! {
rom_01 => "cpu_instr/01-special",
rom_02 => "cpu_instr/02-interrupts",
rom_03 => "cpu_instr/03-op sp,hl",
rom_04 => "cpu_instr/04-op r,imm",
rom_05 => "cpu_instr/05-op rp",
rom_06 => "cpu_instr/06-ld r,r",
rom_07 => "cpu_instr/07-jr,jp,call,ret,rst",
rom_08 => "cpu_instr/08-misc instrs",
rom_09 => "cpu_instr/09-op r,r",
rom_10 => "cpu_instr/10-bit ops",
rom_11 => "cpu_instr/11-op a,(hl)",
rom_12 => "timing/halt_bug",
rom_13 => "timing/instr_timing",
rom_14 => "timing/interrupt_time",
rom_15 => "timing/mem_timing",
rom_16 => "timing/mem_timing_2"
}
