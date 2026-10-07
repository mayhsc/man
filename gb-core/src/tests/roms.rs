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
rom_01 => "01-special",
rom_02 => "02-interrupts",
rom_03 => "03-op sp,hl",
rom_04 => "04-op r,imm",
rom_05 => "05-op rp",
rom_06 => "06-ld r,r",
rom_07 => "07-jr,jp,call,ret,rst",
rom_08 => "08-misc instrs",
rom_09 => "09-op r,r",
rom_10 => "10-bit ops",
rom_11 => "11-op a,(hl)",}
