use ast::{Expression, Expression::DummyExpression, Statement, Statement::LetStatement};
use lexer::Lexer;
use parser::Parser;

#[test]
fn test_let_statements() {
    let inp = "let x = 5;
    let y = 10;
    let foobar = 838383;";

    let expected = vec![
        LetStatement("x".to_string(), DummyExpression),
        LetStatement("y".to_string(), DummyExpression),
        LetStatement("foobar".to_string(), DummyExpression),
    ];

    test_parser(inp, expected);
}

fn test_parser(inp: &str, expected: Vec<Statement>) {
    let lexer = Lexer::new(inp);
    let parser = Parser::new(lexer);

    let program = parser.parse();
    println!("{:?}", program.statements);

    for (i, exp_statement) in expected.iter().enumerate() {
        assert_eq!(*exp_statement, program.statements[i]);
    }
}
