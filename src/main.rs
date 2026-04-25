use knit::codegen::generate_c;
use knit::lexer::tokenize;
use knit::parser::parse;
use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: knit <source.knit>");
        std::process::exit(1);
    }

    let source = fs::read_to_string(&args[1]).expect("Failed to read source file");
    let tokens = tokenize(source);
    let stmts = parse(tokens);
    let c_code = generate_c(stmts);

    let output_c = "output.c";
    fs::write(output_c, c_code).expect("Failed to write output.c");

    let status = Command::new("cc")
        .arg(output_c)
        .arg("-o")
        .arg("program")
        .status()
        .expect("Failed to compile C code");

    if status.success() {
        println!("Compiled successfully: ./program");
    } else {
        eprintln!("Compilation failed");
    }
}
