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
    let mut env = Env::new();

    loop {
        output.write(PROMPT.as_bytes()).unwrap();
        output.flush().unwrap();

        let mut buf = String::new();
        input.read_line(&mut buf).unwrap();

        let (evaluated_env, output) = handle_input(env, &mut output, buf);
        output.expect("no errors for as_bytes()");

        env = evaluated_env;
    }
}

fn handle_input<W>(
    env: Env,
    output: &mut W,
    s: String,
) -> (Env, std::result::Result<usize, std::io::Error>)
where
    W: Write,
{
    // TODO cleanup output
    match Parser::new(Lexer::new(&s)).parse() {
        Ok(program) => {
            let evaluated_env = program.evaluate(env);
            let output_str = evaluated_env.get_result();

            (
                evaluated_env,
                output.write(format!("{:?}\n", output_str).as_bytes()),
            )
        }
        Err(e) => (env, output.write(format!("{:?}\n", e).as_bytes())),
    }
}
