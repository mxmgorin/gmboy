use crate::common::{run_test_case, Sm83TestCase};

mod common;

#[test]
fn test_sm83_case_41() {
    let test_cases = Sm83TestCase::load_opcode(41);

    for test_case in test_cases.iter() {
        run_test_case(test_case);
    }
}

#[test]
fn test_sm83_case_static() {
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

    run_test_case(&test_case);
}
