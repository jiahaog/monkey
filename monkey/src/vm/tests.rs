use super::*;

#[test]
fn test_integer_arithmetic() {
    let tests = vec![
        ("1", Object::Integer(1)),
        ("2", Object::Integer(2)),
        ("1 + 2", Object::Integer(3)),
    ];

    for (inp, expected) in tests {
        let vm = Vm::new();
        let object = vm.run(inp).unwrap();

        assert_eq!(object, expected);
    }
}
