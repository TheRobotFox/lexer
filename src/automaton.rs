use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::regex::{Regexpr, Pattern, Term};

#[derive(Default)]
pub struct NDAState {
    pub end: u16,
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

    // append Regex Pattern into NDA states
    // prev_ends are all prevoius end states, which are to be connected to the next pattern
    // returns the Set of new end states, if pattern is transparent it will also include carry_over
    fn add_pattern(&mut self, regex: Pattern, prev_ends: &HashSet<usize>) -> HashSet<usize> {
        match regex {
            Pattern::Terminal(t) => {

                // create new State
                let state_idx = self.states.len();
                self.states.push(NDAState::default());

                // connect all prevoius EndStates to new State via <t> Terminal
                prev_ends.iter().for_each(|&i: &usize| {
                              self.states.get_mut(i) .unwrap()
                                     .next.entry(t.clone()).or_default().insert(state_idx);});

                HashSet::from([state_idx])
            }
            Pattern::Or(a, b) => {

                // evaluate pattern seperatly
                // merge end states of both

                let mut a = self.add_pattern(*a, prev_ends);
                a.extend(self.add_pattern(*b, prev_ends));
                a
            }
            Pattern::Group(exprs) => {

                // eval Expression sequencally
                // fold over the end states

                exprs.into_iter().fold(prev_ends.clone(), |carry, expr| self.process_regexpr(expr, carry))
            }
        }
    }

    fn process_regexpr(&mut self, expr: Regexpr, prev_ends: HashSet<usize>) -> HashSet<usize> {

        // if pattern loops, capture pattern "entrance" using temporary state <loop_point>
        // instead of previous EndStates, which might be dirty (include unwanted transitions)
        let start = if expr.looping {
            let loop_point = HashSet::from([self.states.len()]);
            self.states.push(NDAState::default());
            loop_point
        } else {prev_ends.clone()};

        let mut out = self.add_pattern(expr.pattern.clone(), &start); // match

        // if looping pattern copy captured <loop_state> transitions to prevoius EndStates
        // and new EndStates, which will therefore be able to reenter the pattern
        if expr.looping {
            out.union(&prev_ends).for_each(|&s|{
                let loop_point = *start.iter().next().unwrap();
                let entries = self.states.get(loop_point).unwrap().next.clone();
                self.states.get_mut(s).unwrap().next.extend(entries)
            });
        }

        // if the pattern is transparent/optional prevoius EndStates remain EndStates
        if expr.transparent {
            out.extend(prev_ends);
        }
        out
    }


    // Create Map with disjunctive Transition Terms
    // The character Sets for each transition should have no overlapping elements
    fn disjunct_terms(transitions: BTreeMap<Term, BTreeSet<usize>>) -> BTreeMap<Term, BTreeSet<usize>> {
        transitions.into_iter().fold(HashMap::new(), |mut acc, (term_insert, target_insert)| {

            let check_for = term_insert.get_chars();

            // collect intersecting Terms
            let mut intersections: Vec<_> = acc.iter().filter(
                |pair| pair.0.get_chars().iter().any(|c| check_for.contains(c))).collect();

            if intersections.is_empty() {
                acc.insert(term_insert, target_insert);
            } else {

                // remove intersecting Transitions
                acc.retain(|(t1,_)| !intersections.iter().any(|(t2,_)| t1==t2));

                // create disjunctive transitions
                for (term, target) in intersections {
                    match term_insert {
                        Term::Char(c) => {
                            acc.insert(key, value)
                        }
                    }
                }
            }

        })
    }

    pub fn new(regex: Pattern) -> NDA {

        // create NDA with single start state <0>
        let start = HashSet::from([0]);
        let mut nda = NDA{states: vec![NDAState::default()], start: start.clone()};


        // insert regex pattern NDA
        let end_states = nda.add_pattern(regex, &start);

        // mark end states
        end_states.into_iter().for_each(|s| nda.states.get_mut(s).unwrap().end = 1);

        nda
    }
    pub fn get_dot_script(&self) -> String {
        let preamble = "digraph NDA {rank=lr\n\"\" [shape=none]\n";

        let states = 0..self.states.len();

        // declare all states
        // use [doublecircle] style for end states
        // use [circle] syle for non end states
        // draw Arrow to start states
        let styles = states.clone().map(|idx| (idx, self.states.get(idx).unwrap().end)).fold(String::new(), |acc, (idx, end)|{
            let res = if end>0 {
                acc + &format!("{} [shape=doublecircle,label={}]\n", idx, end)
            } else {
                acc + &format!("{} [shape=circle,label=\"\"]\n", idx)
            };
            if self.start.contains(&idx) {
                res + &format!("\"\" -> {}\n", idx)
            } else {res}
        });

        // for all states, print all transitions with label
        let transitions = states.fold(String::new(), |acc, state_idx|{
            let state_next = &self.states.get(state_idx).unwrap().next;
            acc + &state_next.iter().fold(String::new(), |acc, (term, targets)|{
                acc + &targets.iter().fold("".to_owned(), |s, t| s+&format!("{} -> {} [label=\"{}\"]", state_idx, t, term) + "\n")
            }) + "\n"
        });

        preamble.to_owned() + &styles + &transitions + "\n}"
    }



    pub fn extend(&mut self, mut other: NDA) {

        let offset = self.states.len();

        // insert additional start
        self.start.insert(offset);

        // increment end states to keep track of the different NDAs
        self.states.iter_mut().for_each(|state| if state.end!=0 {state.end+=1;});

        other.states.iter_mut().for_each(|state| {
                // apply offset to transitions
                state.next.iter_mut().for_each(|(_, targets)| *targets = targets.iter().map(|i|i+offset).collect())
            });

        // append new states
        self.states.append(&mut other.states);
    }
}

pub struct DFAState {
    end: BTreeSet<u16>,
    next: BTreeMap<Term, usize>
}

pub struct DFA {
    states: Vec<DFAState>,
    start: usize
}

impl DFA {


    pub fn new(nda: NDA) -> DFA {

    }
}
