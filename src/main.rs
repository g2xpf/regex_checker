extern crate regex_checker;

use regex_checker::lexer::Lexer;

fn main() {
    let s = ".*|a.*";
    let l = Lexer::new(s);
    println!("{:?}", l.tokens());
}
