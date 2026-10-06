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
    rom_01_special => "01-special",
}
