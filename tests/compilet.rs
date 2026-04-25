use knit::codegen::gen_c::CGen;
use knit::lexer::tokenize;
use knit::parser::parse;
use std::fs;
use std::fs::File;
use std::io::Write;

#[test]
fn main() -> std::io::Result<()> {
    println!("\x1b[36mTest code gen\x1b[0m");
    let code = fs::read_to_string("tests/test.knit").expect("Failed to read test.knit");
    println!("\x1b[35mTest code:\n\x1b[0m{}", code);

    let tokens = tokenize(code);

    let stmts = parse(tokens);

    println!("\x1b[36mGenerated code (C):\x1b[0m");
    let c_code = CGen::new(stmts).generate();
    println!("{}", c_code);

    let mut c_file = File::create("tests/main.c")?;
    c_file.write_all(c_code.as_bytes())?;

    Ok(())
}
