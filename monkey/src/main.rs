extern crate monkey;

use monkey::{Interpreter, InterpreterResult};
use std::io;
use std::io::{BufRead, Write};

const PROMPT: &str = ">>> ";

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    Repl::new(stdin.lock(), stdout.lock()).start();
}

struct Repl<R: BufRead, W: Write> {
    interpreter: Interpreter,
    reader: R,
    writer: W,
}

impl<R: BufRead, W: Write> Repl<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            interpreter: monkey::Interpreter::new(),
            reader: reader,
            writer: writer,
        }
    }

    pub fn start(&mut self) {
        loop {
            self.writer.write(PROMPT.as_bytes()).unwrap();
            self.writer.flush().unwrap();

            let mut buf = String::new();
            self.reader.read_line(&mut buf).unwrap();

            let output = handle_input(&mut self.interpreter, &mut self.writer, buf);
            output.expect("no errors for as_bytes()");
        }
    }
}

fn handle_input<W>(
    interpreter: &mut Interpreter,
    output: &mut W,
    s: String,
) -> std::result::Result<usize, std::io::Error>
where
    W: Write,
{
    let InterpreterResult { stdout, result } = interpreter.evaluate(s);
    if stdout.len() > 0 {
        output.write(format!("{}\n", stdout).as_bytes())?;
    };

    match result {
        Ok(object) => output.write(format!("{}\n", object).as_bytes()),
        Err(e) => output.write(format!("{}\n", e).as_bytes()),
    }
}
