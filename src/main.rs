mod larp;

fn main() {
    let input = "let x = 42 + y";
    let tokens = larp::tokenizer::tokenize(input);
    println!("{:?}", tokens);
}