use rusty_gb_emu::debugger::{CpuLogType, Debugger};
use crate::common::{set_up_cpu, Sm83TestCase};

mod common;

#[test]
fn test_sm83_case() {
    let json_data = r#"
    {
        "name": "41 0000",
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
    let mut cpu = set_up_cpu(&test_case);
    let mut debugger = Some(Debugger::new(CpuLogType::Assembly, false));
    cpu.step(&mut debugger).unwrap();

    test_case.assert_final_state(&cpu);
}
