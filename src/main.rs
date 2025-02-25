use logos::Logos;

mod parse;
mod automaton;
mod regex;
use automaton::NDA;
use crate::parse::*;


fn regex_to_nda(inp: &str) -> NDA {
    let lex = Token::lexer(inp);

    let regex = Parser::parse(lex);
    eprintln!("Parser: {:?}", regex);

    NDA::new(regex)
}


fn main() {
    let mut nda = regex_to_nda("abc(d|e)!+");
    nda.extend(regex_to_nda("(ye)*|no"));
    println!("{}", nda.get_dot_script());
}
