extern crate monkey;

use monkey::vm;

// TODO Remove this
// Rust produces dead code warnings when we have symbols in modules but do not
// use them in lib.rs or as a dependency.
struct UnusedImport(Option<vm::Vm>);

#[test]
fn test_unused_import() {
    let _ = UnusedImport(None);
}
