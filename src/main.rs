extern crate monkey;

use monkey::repl;
use std::io;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    repl::start(stdin.lock(), stdout.lock());
}
