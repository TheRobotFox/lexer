use std::collections::BTreeSet;
use std::boxed::Box;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Term {
    pub chars: BTreeSet<char>,
    pub negate: bool
}


#[derive(Debug, Clone)]
pub enum Pattern{
    Or(Box<Pattern>, Box<Pattern>),
    Group(Vec<Regexpr>),
    Terminal(Term)
}
#[derive(Debug, Clone)]
pub struct Regexpr{
    pub pattern: Pattern,
    pub looping: bool,
    pub transparent: bool
}
