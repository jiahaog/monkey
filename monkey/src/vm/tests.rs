use super::*;

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
    ];

    for (inp, expected) in tests {
        let mut vm = Vm::new();
        let _ = vm.run(Vec::new(), inp).unwrap();

        assert_eq!(vm.last_popped().unwrap(), &expected.into());
    }
}

#[test]
fn test_boolean() {
    let tests = vec![("true", true), ("false", false)];

    for (inp, expected) in tests {
        let mut vm = Vm::new();
        let _ = vm.run(Vec::new(), inp).unwrap();

        assert_eq!(vm.last_popped().unwrap(), &expected.into());
    }
}
