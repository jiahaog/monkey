use crate::lexer::Lexer;
use crate::object::Env;
use crate::parser::Parser;
use std::io::{BufRead, Write};

const PROMPT: &str = ">>> ";

pub fn start<R, W>(mut input: R, mut output: W)
where
    R: BufRead,
    W: Write,
{
    let mut env = Env::new();

    loop {
        output.write(PROMPT.as_bytes()).unwrap();
        output.flush().unwrap();

        let mut buf = String::new();

        input.read_line(&mut buf).unwrap();
        handle_input(&mut env, &mut output, buf).unwrap();
    }
}

fn handle_input<W>(
    env: &mut Env,
    output: &mut W,
    s: String,
) -> std::result::Result<usize, std::io::Error>
where
    W: Write,
{
    // TODO cleanup output
    match Parser::new(Lexer::new(&s)).parse() {
        Ok(program) => {
            let result = program.evaluate(env);
            output.write(format!("{:?}\n", result).as_bytes())
        }
        Err(e) => output.write(format!("{:?}\n", e).as_bytes()),
    }
}
