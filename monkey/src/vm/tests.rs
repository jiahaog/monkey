use super::*;

#[test]
fn test_integer_arithmetic() {
    let tests = vec![
        ("1", Object::Integer(1)),
        ("2", Object::Integer(2)),
        ("1 + 2", Object::Integer(3)),
    ];

    for (inp, expected) in tests {
        let mut vm = Vm::new();
        let _ = vm.run(Vec::new(), inp).unwrap();

        assert_eq!(vm.last_popped().unwrap(), &expected);
    }
}
