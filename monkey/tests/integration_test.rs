extern crate monkey;

use monkey::object::Object;
use monkey::vm::Vm;

#[test]
fn vm_smoke_test() {
    let mut vm = Vm::new();
    let _ = vm.run(Vec::new(), "1 + 2").unwrap();
    assert_eq!(&Object::Integer(3), vm.last_popped().unwrap());
}
