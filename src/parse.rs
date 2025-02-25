use logos::Logos;
#[derive(Logos, Debug, PartialEq, PartialOrd)]
#[logos(skip " ")]pub enum Token {
	EOF,
	#[regex(r#"\\[\[\]\(\)\.\\\+\*\|\?]"#)]
	L1,
	#[token(r#"["#)]
	L2,
	#[token(r#"*"#)]
	L3,
	#[token(r#"|"#)]
	L4,
	#[token(r#"("#)]
	L5,
	#[token(r#")"#)]
	L6,
	#[token(r#"?"#)]
	L7,
	#[regex(r#"[^\[\]\(\)\.\\\+\*\|\?]"#)]
	L8,
	#[token(r#"[^"#)]
	L9,
	#[token(r#"+"#)]
	L10,
	#[token(r#"]"#)]
	L11,
	#[token(r#"-"#)]
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
		[53, 36, 40, 0, 0, 2, 0, 0, 35, 37, 0, 0, 0], 
		[53, 3, 18, 0, 0, 16, 0, 0, 17, 4, 0, 0, 0], 
		[53, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0], 
		[53, 5, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0], 
		[53, -1, 0, 0, 0, 0, 0, 0, -1, 0, 0, -1, -1], 
		[53, -2, 0, 0, 0, 0, 0, 0, -2, 0, 0, -2, -2], 
		[53, -3, 0, 0, 0, 0, 0, 0, -3, 0, 0, -3, 0], 
		[53, 5, 0, 0, 0, 0, 0, 0, 6, 0, 0, 9, 0], 
		[53, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, 0, 0], 
		[53, -4, 0, 0, 0, 0, 0, 0, -4, 0, 0, -4, 11], 
		[53, 12, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0], 
		[53, -1, 0, 0, 0, 0, 0, 0, -1, 0, 0, -1, 0], 
		[53, -2, 0, 0, 0, 0, 0, 0, -2, 0, 0, -2, 0], 
		[53, -6, 0, 0, 0, 0, 0, 0, -6, 0, 0, -6, 0], 
		[53, -7, 0, 0, 0, 0, 0, 0, -7, 0, 0, -7, 0], 
		[53, 3, 18, 0, 0, 16, 0, 0, 17, 4, 0, 0, 0], 
		[53, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 0, 0], 
		[53, 5, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0], 
		[53, 5, 0, 0, 0, 0, 0, 0, 6, 0, 0, 20, 0], 
		[53, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, 0, 0], 
		[53, -12, -12, 23, -12, -12, -12, 24, -12, -12, 22, 0, 0], 
		[53, -9, -9, 0, -9, -9, -9, 0, -9, -9, 0, 0, 0], 
		[53, -10, -10, 0, -10, -10, -10, 0, -10, -10, 0, 0, 0], 
		[53, -11, -11, 0, -11, -11, -11, 0, -11, -11, 0, 0, 0], 
		[53, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, 0, 0], 
		[53, -15, -15, 0, -15, -15, -15, 0, -15, -15, 0, 0, 0], 
		[53, 3, 18, 0, -16, 16, -16, 0, 17, 4, 0, 0, 0], 
		[53, -17, -17, 0, -17, -17, -17, 0, -17, -17, 0, 0, 0], 
		[53, 0, 0, 0, 30, 0, 32, 0, 0, 0, 0, 0, 0], 
		[53, 3, 18, 0, 0, 16, 0, 0, 17, 4, 0, 0, 0], 
		[53, 3, 18, 0, -18, 16, -18, 0, 17, 4, 0, 0, 0], 
		[53, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, 0], 
		[53, 0, 0, 0, 30, 0, 34, 0, 0, 0, 0, 0, 0], 
		[-13, -13, -13, -13, -13, -13, 0, -13, -13, -13, -13, 0, 0], 
		[-2, -2, -2, -2, -2, -2, 0, -2, -2, -2, -2, 0, 0], 
		[-1, -1, -1, -1, -1, -1, 0, -1, -1, -1, -1, 0, 0], 
		[53, 5, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0], 
		[53, 5, 0, 0, 0, 0, 0, 0, 6, 0, 0, 39, 0], 
		[-5, -5, -5, -5, -5, -5, 0, -5, -5, -5, -5, 0, 0], 
		[53, 5, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0], 
		[53, 5, 0, 0, 0, 0, 0, 0, 6, 0, 0, 42, 0], 
		[-8, -8, -8, -8, -8, -8, 0, -8, -8, -8, -8, 0, 0], 
		[-15, -15, -15, 0, -15, -15, 0, 0, -15, -15, 0, 0, 0], 
		[-12, -12, -12, 45, -12, -12, 0, 46, -12, -12, 47, 0, 0], 
		[-10, -10, -10, 0, -10, -10, 0, 0, -10, -10, 0, 0, 0], 
		[-11, -11, -11, 0, -11, -11, 0, 0, -11, -11, 0, 0, 0], 
		[-9, -9, -9, 0, -9, -9, 0, 0, -9, -9, 0, 0, 0], 
		[-16, 36, 40, 0, -16, 2, 0, 0, 35, 37, 0, 0, 0], 
		[-14, -14, -14, -14, -14, -14, 0, -14, -14, -14, -14, 0, 0], 
		[-17, -17, -17, 0, -17, -17, 0, 0, -17, -17, 0, 0, 0], 
		[53, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0], 
		[53, 36, 40, 0, 0, 2, 0, 0, 35, 37, 0, 0, 0], 
		[-18, 36, 40, 0, -18, 2, 0, 0, 35, 37, 0, 0, 0], 
	];

	const GOTO: [ [usize; 18]; 53] = [
		[48, 48, 0, 0, 43, 0, 0, 43, 42, 42, 42, 42, 43, 43, 47, 50, 47, 50], 
		[24, 24, 0, 0, 20, 0, 0, 20, 25, 25, 25, 25, 20, 20, 26, 32, 26, 32], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[9, 9, 7, 6, 0, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[9, 9, 0, 14, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[13, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[24, 24, 0, 0, 20, 0, 0, 20, 25, 25, 25, 25, 20, 20, 26, 28, 26, 28], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[9, 9, 18, 6, 0, 6, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[9, 9, 0, 14, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[24, 24, 0, 0, 20, 0, 0, 20, 27, 27, 27, 27, 20, 20, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[24, 24, 0, 0, 20, 0, 0, 20, 25, 25, 25, 25, 20, 20, 30, 0, 30, 0], 
		[24, 24, 0, 0, 20, 0, 0, 20, 27, 27, 27, 27, 20, 20, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[9, 9, 37, 6, 0, 6, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[9, 9, 0, 14, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[9, 9, 40, 6, 0, 6, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[9, 9, 0, 14, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[48, 48, 0, 0, 43, 0, 0, 43, 49, 49, 49, 49, 43, 43, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
		[48, 48, 0, 0, 43, 0, 0, 43, 42, 42, 42, 42, 43, 43, 52, 0, 52, 0], 
		[48, 48, 0, 0, 43, 0, 0, 43, 49, 49, 49, 49, 43, 43, 0, 0, 0, 0], 
	];

	fn reduction0(mut s: &str) -> char {s.chars().nth(1).unwrap()} 
	fn reduction1(mut s: &str) -> char {s.chars().next().unwrap()} 
	fn reduction2(mut s: BTreeSet<char>) -> BTreeSet<char> {s} 
	fn reduction3(mut c: char) -> BTreeSet<char> {BTreeSet::from([c])} 
	fn reduction4(mut s: BTreeSet<char>) -> Pattern {Pattern::Terminal(Term::NSet(s))} 
	fn reduction5(mut a: char, mut b: char) -> BTreeSet<char> {(a..=b).collect()} 
	fn reduction6(mut stack: BTreeSet<char>, mut s: BTreeSet<char>) -> BTreeSet<char> {stack.union(&s); stack} 
	fn reduction7(mut s: BTreeSet<char>) -> Pattern {Pattern::Terminal(Term::Set(s)) } 
	fn reduction8(mut t: Pattern) -> Regexpr {Regexpr{pattern: t, looping: true, transparent: false}} 
	fn reduction9(mut t: Pattern) -> Regexpr {Regexpr{pattern: t, looping: true, transparent: true}} 
	fn reduction10(mut t: Pattern) -> Regexpr {Regexpr{pattern: t, looping: false, transparent: true}} 
	fn reduction11(mut t: Pattern) -> Regexpr {Regexpr{pattern: t, looping: false, transparent: false}} 
	fn reduction12(mut p: Pattern) -> Pattern {p                               } 
	fn reduction13(mut s: char) -> Pattern {Pattern::Terminal(Term::Char(s))} 
	fn reduction14(mut e: Regexpr) -> Vec<Regexpr> {vec![e]} 
	fn reduction15(mut t: Vec<Regexpr>) -> Pattern {Pattern::Group(t)                                    } 
	fn reduction16(mut stack: Vec<Regexpr>, mut e: Regexpr) -> Vec<Regexpr> {stack.push(e); stack} 
	fn reduction17(mut a: Pattern, mut b: Vec<Regexpr>) -> Pattern {Pattern::Or(Box::new(a),Box::new(Pattern::Group(b))) } 

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
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a1 = pop!(parser, T3);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction4(a1));
			}
			-6 => {
				let a2 = pop!(parser, T2);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T2);
 				let _ = parser.state_stack.pop();
				push!(parser, T3, Self::reduction5(a0, a2));
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
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction8(a0));
			}
			-10 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction9(a0));
			}
			-11 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction10(a0));
			}
			-12 => {
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T5, Self::reduction11(a0));
			}
			-13 => {
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a1 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction12(a1));
			}
			-14 => {
				let a0 = pop!(parser, T2);
 				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction13(a0));
			}
			-15 => {
				let a0 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				push!(parser, T6, Self::reduction14(a0));
			}
			-16 => {
				let a0 = pop!(parser, T6);
 				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction15(a0));
			}
			-17 => {
				let a1 = pop!(parser, T5);
 				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T6);
 				let _ = parser.state_stack.pop();
				push!(parser, T6, Self::reduction16(a0, a1));
			}
			-18 => {
				let a2 = pop!(parser, T6);
 				let _ = parser.state_stack.pop();
				let _ = parser.parse_stack.pop();
				let _ = parser.state_stack.pop();
				let a0 = pop!(parser, T4);
 				let _ = parser.state_stack.pop();
				push!(parser, T4, Self::reduction17(a0, a2));
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
	T5(Regexpr),
	T6(Vec<Regexpr>),
	T3(BTreeSet<char>),
	T2(char),
	T4(Pattern),
	T1(&'a str)
}

