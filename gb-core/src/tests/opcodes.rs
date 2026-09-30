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
    op_7f => "7F",
    op_78 => "78",
    op_79 => "79",
    op_7a => "7A",
    op_7b => "7B",
    op_7c => "7C",
    op_7d => "7D",

}
