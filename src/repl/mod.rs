use crate::eval::Env;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::{BufRead, Write};

const PROMPT: &str = ">>> ";

pub fn start<R, W>(mut input: R, mut output: W)
where
    R: BufRead,
    W: Write,
{
    let env = Env::new();

    loop {
        output.write(PROMPT.as_bytes()).unwrap();
        output.flush().unwrap();

        let mut buf = String::new();
        input.read_line(&mut buf).unwrap();

        let output = handle_input(env.clone(), &mut output, buf);
        output.expect("no errors for as_bytes()");
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
