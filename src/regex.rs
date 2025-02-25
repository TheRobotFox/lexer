use std::collections::BTreeSet;
use std::boxed::Box;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Term{
    NSet(BTreeSet<char>),
    Set(BTreeSet<char>),
    Char(char),
}

impl Term {
    pub fn get_chars(&self) -> BTreeSet<char> {
        match self {
            Term::Char(c) => BTreeSet::from([*c]),
            Term::Set(cs) => cs.clone(),
            Term::NSet(cs) => cs.clone()
        }
    }
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
