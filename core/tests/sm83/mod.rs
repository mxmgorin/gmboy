mod util;

use crate::sm83::util::{run_sb_test_cases, run_test_case, Sm83TestCase};
use core::cpu::instructions::opcodes::INSTRUCTIONS_BY_OPCODES;
use core::cpu::instructions::Mnemonic;
use std::time::Duration;

#[test]
fn test_sm83_all() {
    let mut count = 0;

    for (opcode, instruction) in INSTRUCTIONS_BY_OPCODES.iter().enumerate() {
        if Mnemonic::Unknown == instruction.get_mnemonic() {
            continue;
        }

        if opcode == 0xCB {
            count += run_sb_test_cases(false);
            continue;
        }

        let Ok(test_cases) = Sm83TestCase::load_opcode(opcode as u16) else {
            println!("{:02X} no tests cases", opcode);
            continue;
        };

        for test_case in test_cases.iter() {
            run_test_case(test_case, false);
            count += 1;
        }

        println!(
            "{:02X} passed {} 'sm83' test cases",
            opcode,
            test_cases.len()
        );
    }

    println!("passed {count} 'sm83' test cases",);
}

#[test]
fn test_sm83_str_json() {
    println!("{:?}", Duration::default());

    let json_data = r#"
    {
        "name": "41 0000 str json",
        "initial": {
            "pc": 9845,
            "sp": 50643,
            "a": 185,
            "b": 151,
            "c": 101,
            "d": 187,
            "e": 72,
            "f": 160,
            "h": 117,
            "l": 249,
            "ime": 1,
            "ie": 0,
            "ram": [[9845, 65]]
        },
        "final": {
            "a": 185,
            "b": 101,
            "c": 101,
            "d": 187,
            "e": 72,
            "f": 160,
            "h": 117,
            "l": 249,
            "pc": 9846,
            "sp": 50643,
            "ime": 1,
            "ram": [[9845, 65]]
        },
        "cycles": [[9845, 65, "r-m"]]
    }"#;

    let test_case = Sm83TestCase::from_json(json_data);

    run_test_case(&test_case, true);
}
