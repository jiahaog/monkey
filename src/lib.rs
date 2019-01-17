mod ast;
mod eval;
mod lexer;
mod parser;
pub mod repl;
mod token;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
