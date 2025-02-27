use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::regex::{Regexpr, Pattern, Term};

#[derive(Default)]
pub struct NDAState {
    pub end: u16,
    pub next: HashMap<Option<BTreeSet<char>>, BTreeSet<usize>>

}


impl NDAState {

    // Create Map with disjunctive Transition Terms
    // The character Sets for each transition should have no overlapping elements
    pub fn transition(&mut self, insert_term: Term, insert_targets: BTreeSet<usize>) {

        // collect overlapping Terms
        let conflicts: Vec<_> = self.next.iter().filter(
            |(t, _)| t.as_ref().map_or(false, |chars| chars.iter().any(|c| insert_term.chars.contains(c))))
                                                    .map(|(t,s)| (t.clone().unwrap(), s.clone())).collect();

        // remove overlapping Transitions
        self.next.retain(|t1,_| !conflicts.iter().any(|(t2,_)| t1.as_ref().map_or(false, |t1|t1==t2)));

        let mut insert_chars = insert_term.chars;
        // create disjunctive transitions
        for (char_set, mut targets) in conflicts {
            let intersection: BTreeSet<char> = char_set.intersection(&insert_chars).cloned().collect();
            let difference: BTreeSet<char> = char_set.difference(&insert_chars).cloned().collect();

            let (exclude, include) = if insert_term.negate {
                (intersection, difference)
            } else {
                (difference, intersection)
            };

            self.next.insert(Some(exclude), targets.clone());
            targets.extend(insert_targets.clone());
            self.next.insert(Some(include), targets);

            insert_chars.retain(|i| !char_set.contains(i)); // continue with others
        }

        // insert rest
        // FIXME if NonGroup add to all others
        if !insert_chars.is_empty() {
            self.next.insert(Some(insert_chars),
                    if insert_term.negate {
                        self.next.get(&None).cloned().unwrap_or_default()
                    } else {
                        insert_targets.clone()
                    });
        }

        // combine with all negated targets
        if insert_term.negate {
            self.next.entry(None).or_default().extend(insert_targets);
        }
    }
}

pub struct NDA {
    states: Vec<NDAState>,
    start: HashSet<usize>
}

fn set_to_string(set: BTreeSet<char>) -> String {
    let content = set.iter().fold(String::new(), |acc, i| acc+&i.to_string());
    if set.len()==1 {
        content
    } else {
        format!("[{}]", content)

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
                prev_ends.iter().for_each(|&i| {
                    self.states.get_mut(i).unwrap().transition(t.clone(), BTreeSet::from([state_idx]));
                });
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
                acc + &targets.iter().fold("".to_owned(), |s, t| s+&format!("{} -> {} [label=\"{}\"]", state_idx, t,
                                                                            term.clone().map_or(String::from("<otherwise>"), |t| set_to_string(t))) + "\n")
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

// TODO disjunctive_transition in DFA

// impl DFA {


//     pub fn new(nda: NDA) -> DFA {

//     }
// }
