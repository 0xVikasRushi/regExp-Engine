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
