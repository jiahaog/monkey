extern crate monkey;

use monkey::object::Object;
use monkey::vm::Vm;

#[test]
fn vm_smoke_test() {
    let vm = Vm::new();
    let result = vm.run("1 + 2").unwrap();
    assert_eq!(Object::Integer(3), result);
}
