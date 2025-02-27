
use std::collections::BTreeSet;
use std::boxed::Box;
use crate::regex::*;
use logos::Logos;
#[derive(Logos, Debug, PartialEq, PartialOrd)]
#[logos(skip " ")]pub enum Token {
	EOF,
	#[regex(r#"\\[\[\]\(\)\.\\\+\*\|\?]"#)]
	L1,
	#[token(r#"["#)]
	L2,
	#[token(r#"[^"#)]
	L3,
	#[token(r#"?"#)]
	L4,
	#[regex(r#"[^\[\]\(\)\.\\\+\*\|\?]"#)]
	L5,
	#[token(r#"("#)]
	L6,
	#[token(r#"*"#)]
	L7,
	#[token(r#"+"#)]
	L8,
	#[token(r#"|"#)]
	L9,
	#[token(r#"-"#)]
	L10,
	#[token(r#"]"#)]
	L11,
	#[token(r#")"#)]
	L12,
}

pub struct Parser<'a> {
	parse_stack: Vec<Types<'a>>,
	state_stack: Vec<usize>,
	lexer: logos::Lexer<'a, Token>
}

macro_rules! pop{
	($self:ident, $t:ident) => {
		match $self.parse_stack.pop().unwrap() {
			Types::$t(t) =>t,
			o@_ => {eprintln!("Expected: {:?} got {:?}", stringify!($t), o);
            unreachable!()}
		}
	}
}macro_rules! push {
	($self:ident, $t:ident, $e:expr) => {
		$self.parse_stack.push(Types::$t($e));
	}
}
impl<'a> Parser<'a> {
	const ACTION: [ [isize; 13]; 53] = [
		[53, 2, 15, 3, 0, 18, 19, 0, 0, 0, 0, 0, 0], 
		[-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0], 
		[53, 4, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0], 
		[53, -1, 0, 0, 0, -1, 0, 0, 0, 0, -1, -1, 0], 
		[53, -2, 0, 0, 0, -2, 0, 0, 0, 0, -2, -2, 0], 
		[53, -3, 0, 0, 0, -3, 0, 0, 0, 0, 0, -3, 0], 
		[53, -4, 0, 0, 0, -4, 0, 0, 0, 0, 8, -4, 0], 
		[53, 9, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0], 
		[53, -1, 0, 0, 0, -1, 0, 0, 0, 0, 0, -1, 0], 
		[53, -2, 0, 0, 0, -2, 0, 0, 0, 0, 0, -2, 0], 
		[53, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, -5, 0], 
		[53, 4, 0, 0, 0, 5, 0, 0, 0, 0, 0, 13, 0], 
		[-6, -6, -6, -6, -6, -6, -6, -6, -6, -6, 0, 0, 0], 
		[53, -7, 0, 0, 0, -7, 0, 0, 0, 0, 0, -7, 0], 
		[53, 4, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0], 
		[53, 4, 0, 0, 0, 5, 0, 0, 0, 0, 0, 17, 0], 
		[-8, -8, -8, -8, -8, -8, -8, -8, -8, -8, 0, 0, 0], 
		[-2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 0, 0, 0], 
		[53, 23, 25, 20, 0, 24, 28, 0, 0, 0, 0, 0, 0], 
		[53, 4, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0], 
		[53, 4, 0, 0, 0, 5, 0, 0, 0, 0, 0, 22, 0], 
		[53, -6, -6, -6, -6, -6, -6, -6, -6, -6, 0, 0, -6], 
		[53, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, -1], 
		[53, -2, -2, -2, -2, -2, -2, -2, -2, -2, 0, 0, -2], 
		[53, 4, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0], 
		[53, 4, 0, 0, 0, 5, 0, 0, 0, 0, 0, 27, 0], 
		[53, -8, -8, -8, -8, -8, -8, -8, -8, -8, 0, 0, -8], 
		[53, 23, 25, 20, 0, 24, 28, 0, 0, 0, 0, 0, 0], 
		[53, -9, -9, -9, 0, -9, -9, 0, 0, -9, 0, 0, -9], 
		[53, -13, -13, -13, 31, -13, -13, 33, 32, -13, 0, 0, -13], 
		[53, -11, -11, -11, 0, -11, -11, 0, 0, -11, 0, 0, -11], 
		[53, -10, -10, -10, 0, -10, -10, 0, 0, -10, 0, 0, -10], 
		[53, -12, -12, -12, 0, -12, -12, 0, 0, -12, 0, 0, -12], 
		[53, -15, -15, -15, -15, -15, -15, -15, -15, -15, 0, 0, -15], 
		[53, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 36], 
		[53, -14, -14, -14, -14, -14, -14, -14, -14, -14, 0, 0, -14], 
		[53, 23, 25, 20, 0, 24, 28, 0, 0, 0, 0, 0, 0], 
		[53, 23, 25, 20, 0, 24, 28, 0, 0, -16, 0, 0, -16], 
		[53, -17, -17, -17, 0, -17, -17, 0, 0, -17, 0, 0, -17], 
		[53, 23, 25, 20, 0, 24, 28, 0, 0, -18, 0, 0, -18], 
		[53, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 42], 
		[-14, -14, -14, -14, -14, -14, -14, -14, -14, -14, 0, 0, 0], 
		[-18, 2, 15, 3, 0, 18, 19, 0, 0, -18, 0, 0, 0], 
		[-13, -13, -13, -13, 47, -13, -13, 46, 45, -13, 0, 0, 0], 
		[-10, -10, -10, -10, 0, -10, -10, 0, 0, -10, 0, 0, 0], 
		[-12, -12, -12, -12, 0, -12, -12, 0, 0, -12, 0, 0, 0], 
		[-11, -11, -11, -11, 0, -11, -11, 0, 0, -11, 0, 0, 0], 
		[-17, -17, -17, -17, 0, -17, -17, 0, 0, -17, 0, 0, 0], 
		[-15, -15, -15, -15, -15, -15, -15, -15, -15, -15, 0, 0, 0], 
		[-9, -9, -9, -9, 0, -9, -9, 0, 0, -9, 0, 0, 0], 
		[53, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0], 
		[53, 2, 15, 3, 0, 18, 19, 0, 0, 0, 0, 0, 0], 
		[-16, 2, 15, 3, 0, 18, 19, 0, 0, -16, 0, 0, 0], 
	];

	const GOTO: [ [usize; 18]; 53] = [
		[48, 48, 0, 0, 0, 43, 0, 43, 42, 49, 49, 49, 49, 43, 43, 50, 42, 50], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[6, 6, 11, 5, 5, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[6, 6, 0, 13, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[6, 6, 15, 5, 5, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[6, 6, 0, 13, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[33, 33, 0, 0, 0, 29, 0, 29, 39, 28, 28, 28, 28, 29, 29, 40, 39, 40], 
		[6, 6, 20, 5, 5, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[6, 6, 0, 13, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[6, 6, 25, 5, 5, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[6, 6, 0, 13, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[33, 33, 0, 0, 0, 29, 0, 29, 39, 28, 28, 28, 28, 29, 29, 34, 39, 34], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[33, 33, 0, 0, 0, 29, 0, 29, 37, 28, 28, 28, 28, 29, 29, 0, 37, 0], 
		[33, 33, 0, 0, 0, 29, 0, 29, 0, 38, 38, 38, 38, 29, 29, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[33, 33, 0, 0, 0, 29, 0, 29, 0, 38, 38, 38, 38, 29, 29, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[48, 48, 0, 0, 0, 43, 0, 43, 0, 47, 47, 47, 47, 43, 43, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[48, 48, 0, 0, 0, 43, 0, 43, 52, 49, 49, 49, 49, 43, 43, 0, 52, 0], 
		[48, 48, 0, 0, 0, 43, 0, 43, 0, 47, 47, 47, 47, 43, 43, 0, 0, 0], 
	];

	fn reduction0(mut s: &str) -> char {s.chars().nth(1).unwrap()} 
	fn reduction1(mut s: &str) -> char {s.chars().next().unwrap()} 
	fn reduction2(mut s: BTreeSet<char>) -> BTreeSet<char> {s} 
	fn reduction3(mut c: char) -> BTreeSet<char> {BTreeSet::from([c])} 
	fn reduction4(mut a: char, mut b: char) -> BTreeSet<char> {(a..=b).collect()} 
	fn reduction5(mut s: BTreeSet<char>) -> Pattern {Pattern::Terminal(Term{chars: s, negate: true})                   } 
	fn reduction6(mut stack: BTreeSet<char>, mut s: BTreeSet<char>) -> BTreeSet<char> {stack.union(&s); stack} 
	fn reduction7(mut s: BTreeSet<char>) -> Pattern {Pattern::Terminal(Term{chars: s, negate: false})                  } 
	fn reduction8(mut e: Regexpr) -> Vec<Regexpr> {vec![e]} 
	fn reduction9(mut t: Pattern) -> Regexpr {Regexpr{pattern: t, looping: true, transparent: false}} 
	fn reduction10(mut t: Pattern) -> Regexpr {Regexpr{pattern: t, looping: false, transparent: true}} 
	fn reduction11(mut t: Pattern) -> Regexpr {Regexpr{pattern: t, looping: true, transparent: true}} 
	fn reduction12(mut t: Pattern) -> Regexpr {Regexpr{pattern: t, looping: false, transparent: false}} 
	fn reduction13(mut p: Pattern) -> Pattern {p                                                                 } 
	fn reduction14(mut s: char) -> Pattern {Pattern::Terminal(Term{chars: BTreeSet::from([s]), negate: false})} 
	fn reduction15(mut a: Pattern, mut b: Vec<Regexpr>) -> Pattern {Pattern::Or(Box::new(a),Box::new(Pattern::Group(b))) } 
	fn reduction16(mut stack: Vec<Regexpr>, mut e: Regexpr) -> Vec<Regexpr> {stack.push(e); stack} 
	fn reduction17(mut t: Vec<Regexpr>) -> Pattern {Pattern::Group(t)                                    } 

    pub fn parse(lex: logos::Lexer<'a, Token>) -> Pattern {
        let mut parser = Self{
            parse_stack: vec![],
            state_stack: vec![0],
            lexer: lex
        };

        let mut token = match parser.lexer.next() {
            Some(Ok(t)) => t as usize,
            Some(Err(e)) => panic!("{:?}", e),
            None => 0
        };

        while parser.state_stack.len()>0 {
            let state = *parser.state_stack.last().unwrap();
            eprintln!("stack: {:?}", parser.parse_stack);
            eprintln!("stack: {:?}", parser.state_stack);
            eprintln!("got: {}:{}", state, token.clone() as usize);
            let task = Self::ACTION[state][token];
            eprintln!("task: {}", task);
            match task {
                0 => panic!("Parsing failed! {:?} {:?}", parser.parse_stack, parser.state_stack),
                53 => break,
			-1 => {
				let a0 = pop!(parser, T1);
 				let _ = parser.state_stack.pop();
				push!(parser, T2, Self::reduction0(a0));
			}
			-2 => {
				let a0 = pop!(parser, T1);
 				let _ = parser.state_stack.pop();
				push!(parser, T2, Self::reduction1(a0));
			}
			-3 => {
				let a0 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				push!(parser, T3, Self::reduction2(a0));
			}
			-4 => {
				let a0 = pop!(parser, T2);
 				let _ = parser.state_stack.pop();
				push!(parser, T3, Self::reduction3(a0));
			}
			-5 => {
				let a2 = pop!(parser, T2);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T2);
 				let _ = parser.state_stack.pop();
				push!(parser, T3, Self::reduction4(a0, a2));
			}
			-6 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a1 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction5(a1));
			}
			-7 => {
				let a1 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				push!(parser, T3, Self::reduction6(a0, a1));
			}
			-8 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a1 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction7(a1));
			}
			-9 => {
				let a0 = pop!(parser, T6);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction8(a0));
			}
			-10 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T6, Self::reduction9(a0));
			}
			-11 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T6, Self::reduction10(a0));
			}
			-12 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T6, Self::reduction11(a0));
			}
			-13 => {
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T6, Self::reduction12(a0));
			}
			-14 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a1 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction13(a1));
			}
			-15 => {
				let a0 = pop!(parser, T2);
 				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction14(a0));
			}
			-16 => {
				let a2 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction15(a0, a2));
			}
			-17 => {
				let a1 = pop!(parser, T6);
 				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction16(a0, a1));
			}
			-18 => {
				let a0 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction17(a0));
			}

                new_state @ _ => {
                    parser.state_stack.push((new_state-1) as usize);
                    push!(parser, T1, parser.lexer.slice());
                    token = match parser.lexer.next() {
                        Some(Ok(t)) => t as usize,
            Some(Err(e)) =>{
                let mut line=0;
                let mut offset=0;
                let span = parser.lexer.span();
                for c in parser.lexer.source()[0..span.end].chars(){
                    if(c=='\n'){
                        offset=0;
                        line+=1;
                    }
                }

                panic!("Unexpected Token {:?} ({:?}) at {}:{}", e, parser.lexer.slice(), line, offset);
            },
                        None => 0
                    };
                    continue;
                }
            }
            while parser.state_stack.len()>0 {
                let prev = *parser.state_stack.last().unwrap();
                let next = Self::GOTO[prev][-(task+1) as usize];
                if next!=0 {
                    parser.state_stack.push(next);
                    break
                }
                parser.state_stack.pop();
            }
        }
        match parser.parse_stack.into_iter().nth(0).unwrap() {
            Types::T4(s) => s,
            t@ _ => panic!("Parsing failed! {:?}", t)

        }
    }
}

#[derive(Debug)]enum Types<'a> {
	T1(&'a str),
	T2(char),
	T3(BTreeSet<char>),
	T4(Pattern),
	T5(Vec<Regexpr>),
	T6(Regexpr)
}

