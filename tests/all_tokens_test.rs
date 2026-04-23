use knit::lexer::tokenize;

#[test]
fn main() {
    println!("\x1b[36mTest tokenize all tokens\x1b[0m");
    let text = "fn for while if else + - / () {} [] ; f32 i32 bool char 'my string' 7363 . , my_id += /= *= -= /* big comnent */ -- litle comment ";
    println!("\x1b[35mTest text: \x1b[0m{}", text);
    let tokens = tokenize(text.to_string());
    for t in tokens {
        println!("{}", t);
    }
}
