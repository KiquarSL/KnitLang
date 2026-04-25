use knit::codegen::gen_c::CGen;
use knit::lexer::tokenize;
use knit::parser::parse;
use std::fs;
#[test]
fn main() {
    println!("\x1b[36mTest code gen\x1b[0m");
    let code = fs::read_to_string("tests/test.knit").expect("REASON");
    let tokens = tokenize(code);
    let stmts = parse(tokens);

    println!("\x1b[36mGenerated code (C):\x1b[0m");
    let c_code = CGen::new(stmts).generate();
    println!("{}", c_code);
}
