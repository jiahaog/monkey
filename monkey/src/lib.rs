mod ast;
mod eval;
mod lexer;
mod parser;
mod token;

use crate::eval::Env;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::{BufRead, Write};

const PROMPT: &str = ">>> ";

pub struct Interpreter<R: BufRead, W: Write> {
    env: Env,
    reader: R,
    writer: W,
}

impl<R: BufRead, W: Write> Interpreter<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            env: Env::new(),
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

            let output = handle_input(self.env.clone(), &mut self.writer, buf);
            output.expect("no errors for as_bytes()");
        }
    }
}

fn handle_input<W>(
    env: Env,
    output: &mut W,
    s: String,
) -> std::result::Result<usize, std::io::Error>
where
    W: Write,
{
    // TODO cleanup output
    match Parser::new(Lexer::new(&s)).parse() {
        Ok(program) => {
            let object = program.evaluate(env);

            output.write(format!("{:?}\n", object).as_bytes())
        }
        Err(e) => output.write(format!("{:?}\n", e).as_bytes()),
    }
}
