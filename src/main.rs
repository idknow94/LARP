use crate::larp::lexer::{Lexer, Token};
mod larp;
fn main() {
    let mut lexer = Lexer::new("let my_variable = 2*5; while my_variable < 10 {my_variable += 1;}");
    while !lexer.end() {
        let tk: Token = lexer.next_token();
        print!("{:?}, {}->{}\n", tk.kind, tk.span.start, tk.span.end);
    }
}