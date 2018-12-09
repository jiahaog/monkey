use lexer::Lexer;
use parser::Parser;
use std::io::{BufRead, Write};

const PROMPT: &str = ">>> ";

pub fn start<R, W>(mut input: R, mut output: W)
where
    R: BufRead,
    W: Write,
{
    loop {
        output.write(PROMPT.as_bytes()).unwrap();
        output.flush().unwrap();

        let mut buf = String::new();

        input.read_line(&mut buf).unwrap();
        handle_input(&mut output, buf)
    }
}

fn handle_input<W>(output: &mut W, s: String)
where
    W: Write,
{
    Lexer::new(&s).for_each(|token| {
        output.write(format!("{:?}", token).as_bytes()).unwrap();
        output.write(b"\n").unwrap();
    });

    // TODO
    let _ = Parser::new(Lexer::new(&s));
}
