use crate::{nfa::CELL, state::State};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct DFA {
    pub in_state: Rc<RefCell<State>>,
    pub out_state: Rc<RefCell<State>>,
}

impl DFA {
    pub fn new() -> DFA {
        DFA {
            in_state: Rc::new(RefCell::new(State::new(false))),
            out_state: Rc::new(RefCell::new(State::new(true))),
        }
    }

    pub fn get_transition_table(nfa_table: HashMap<Uuid, Vec<CELL>>) -> HashMap<Uuid, Vec<CELL>> {
        let ans: HashMap<Uuid, Vec<CELL>> = HashMap::new();
        return ans;
    }

    pub fn test(_string: &str) -> bool {
        return false;
    }
    pub fn get_accepting_states() {}
}

#[cfg(test)]
mod test {
    use crate::nfa::NFA;

    #[test]
    fn test_get_nfa_transition_table() {
        let mut first_nfa = NFA::char("a");
        let mut second_nfa = NFA::char("b");

        let final_nfa = NFA::or_pair(&mut first_nfa, &mut second_nfa);

        let nfa_table = final_nfa.get_transition_table();
    }
}
