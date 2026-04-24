use knit::lexer::tokenize;
use knit::parser::expr::Expr;
use knit::parser::Parser;

fn eval(text: &str) -> Expr {
    let tokens = tokenize(text.to_string());
    println!("\x1b[36mTokens:\x1b[0m");
    for i in &tokens {
        println!("{}", i);
    }
    Parser::new(tokens).expr()
}

#[test]
fn main() {
    println!("\x1b[36mTest calculate expression\x1b[0m");
    let expression = "(2 + 2 *2)/ 3";
    println!("\x1b[35mTest expression: \x1b[0m{:?}", expression);
    let result = eval(expression);
    println!("\x1b[35mResult \x1b[0m{:?}", result);

    println!("\n\x1b[36mTest compare expression\x1b[0m");
    let expression = "1 > 0 && 15 >= 5";
    println!("\x1b[35mTest expression: \x1b[0m{:?}", expression);
    let result = eval(expression);
    println!("\x1b[35mResult \x1b[0m{:?}", result);
}
