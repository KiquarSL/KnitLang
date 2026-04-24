use knit::lexer::tokenize;
use knit::parser::parse;
use std::fs;

#[test]
fn main() {
    println!("\x1b[36mTest parse statements\x1b[0m");
    let code = fs::read_to_string("tests/test.knit").expect("REASON");
    println!("\x1b[35mTest code:\n\x1b[0m{:?}", code);
    println!("\x1b[36mTokens:\x1b[0m");
    let tokens = tokenize(code);
    for i in &tokens {
        println!("{}", i);
    }
    println!("\x1b[36mStatements:\x1b[0m");
    let stmts = parse(tokens);
    if stmts.is_empty() {
        println!("\x1b[34mNo statements\x1b[0m");
    }
    for i in stmts {
        println!("{:#?}", i);
    }
}
