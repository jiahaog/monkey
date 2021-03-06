use super::*;
use crate::object::Object;

#[test]
fn test_integer_arithmetic() {
    let tests = vec![
        ("1", 1),
        ("2", 2),
        ("1 + 2", 3),
        ("1 - 2", -1),
        ("1 * 2", 2),
        ("4 / 2", 2),
        ("50 / 2 * 2 + 10 - 5", 55),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("5 * (2 + 10)", 60),
        // ("if (true) { 10 }", 10),
        // ("if (true) { 10 } else { 20 }", 10),
        // ("if (false) { 10 } else { 20 } ", 20),
        // ("if (1) { 10 }", 10),
        // ("if (1 < 2) { 10 }", 10),
        // ("if (1 < 2) { 10 } else { 20 }", 10),
        // ("if (1 > 2) { 10 } else { 20 }", 20),
    ];

    for (inp, expected) in tests {
        let mut vm = Vm::new();
        let _ = vm.run(Vec::new(), inp).unwrap();

        assert_eq!(&Object::from(expected), vm.last_popped().unwrap());
    }
}

#[test]
fn test_boolean() {
    let tests = vec![
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("true == true", true),
        ("false == false", true),
        ("true == false", false),
        ("true != false", true),
        ("false != true", true),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
    ];

    for (inp, expected) in tests {
        let mut vm = Vm::new();
        let _ = vm.run(Vec::new(), inp).unwrap();

        assert_eq!(&Object::from(expected), vm.last_popped().unwrap());
    }
}
