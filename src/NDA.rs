use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;
use crate::parse::Regexpr;
use crate::Pattern;

use crate::Term;

#[derive(Default)]
pub struct NDAState {
    pub end: bool,
    pub next: BTreeMap<Term, BTreeSet<usize>>

}
pub struct NDA {
    states: Vec<NDAState>,
    start: HashSet<usize>
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::NSet(set) => write!(f, "[^{:?}]", set),
            Term::Set(set) => write!(f, "[{:?}]", set),
            Term::Char(c) => write!(f, "{}", c)
        }
    }
}

impl NDA {

    fn add_pattern(&mut self, regex: Pattern, carry_over: &HashSet<usize>) -> HashSet<usize> {
        match regex {
            Pattern::Terminal(t) => {


                let state_idx = self.states.len();
                self.states.push(NDAState::default());

                carry_over.iter() .for_each(|&i: &usize| {
                              self.states.get_mut(i) .unwrap()
                                     .next.entry(t.clone()).or_default().insert(state_idx);});

                HashSet::from([state_idx])
            }
            Pattern::Or(a, b) => {
                let mut a = self.add_pattern(*a, carry_over);
                a.extend(self.add_pattern(*b, carry_over));
                a
            }
            Pattern::Group(exprs) => {
                exprs.into_iter().rfold(carry_over.clone(), |carry, expr| self.process_regexpr(expr, carry))
            }
        }
    }

    fn process_regexpr(&mut self, expr: Regexpr, carry_over: HashSet<usize>) -> HashSet<usize> {

        let start = if expr.looping {
            let loop_point = HashSet::from([self.states.len()]);
            self.states.push(NDAState::default());
            loop_point
        } else {carry_over.clone()};

        let mut out = self.add_pattern(expr.pattern.clone(), &start); // match

        if expr.transparent {
            out.extend(carry_over);
        }

        if expr.looping {
            out.iter().for_each(|&s|{
                let loop_point = *start.iter().next().unwrap();
                let entries = self.states.get(loop_point).unwrap().next.clone();
                self.states.get_mut(s).unwrap().next.extend(entries)
            });
        }
        out
    }

    pub fn new(regex: Pattern) -> NDA {
        let start = HashSet::from([0]);
        let mut nda = NDA{states: vec![NDAState::default()], start: start.clone()};


        let end_states = nda.add_pattern(regex, &start);

        // mark end states
        end_states.into_iter().for_each(|s| nda.states.get_mut(s).unwrap().end = true);

        nda
    }
    pub fn get_dot_script(&self) -> String {

        (0..self.states.len()).fold("digraph {\n".to_owned(), |code, state_idx|{
            let transitions = &self.states.get(state_idx).unwrap().next;
            code+ &transitions.iter().fold(String::new(), |transistions_code, (term, targets)|{
                transistions_code + &format!("{} -> {:?}: {}", state_idx, targets, term)
            })
        }) + "}"
    }
}
