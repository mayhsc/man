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

    op_47 => "47",
    op_40 => "40",
    op_41 => "41",
    op_42 => "42",
    op_43 => "43",
    op_44 => "44",
    op_45 => "45",

    op_4f => "4F",
    op_48 => "48",
    op_49 => "49",
    op_4a => "4A",
    op_4b => "4B",
    op_4c => "4C",
    op_4d => "4D",

    op_57 => "57",
    op_50 => "50",
    op_51 => "51",
    op_52 => "52",
    op_53 => "53",
    op_54 => "54",
    op_55 => "55",

    op_5f => "5F",
    op_58 => "58",
    op_59 => "59",
    op_5a => "5A",
    op_5b => "5B",
    op_5c => "5C",
    op_5d => "5D",

    op_67 => "67",
    op_60 => "60",
    op_61 => "61",
    op_62 => "62",
    op_63 => "63",
    op_64 => "64",
    op_65 => "65",

    op_6f => "6F",
    op_68 => "68",
    op_69 => "69",
    op_6a => "6A",
    op_6b => "6B",
    op_6c => "6C",
    op_6d => "6D",

     op_06 => "06",
    // op_0e => "0E",
    // op_16 => "16",
    // op_1e => "1E",
    // op_26 => "26",
    // op_2e => "2E",
    // op_36 => "36",
    // op_3e => "3E",
}
