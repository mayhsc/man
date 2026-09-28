
use super::harness::run_file;

macro_rules! opcode_tests {
    ($($fn_name:ident => $file:literal),* $(,)?) => {
        $(
            #[test]
            fn $fn_name() {
                run_file($file);
            }
        )*
    };
}

opcode_tests! {
    op_00 => "00",
    op_01 => "01",
}