extern crate monkey;

use std::io;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    monkey::Interpreter::new(stdin.lock(), stdout.lock()).start();
}
