use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub const EPSILON: &str = "ε";

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub accepting: bool,
    pub transition_map: HashMap<String, Vec<Rc<RefCell<State>>>>,
}

impl State {
    pub fn new(is_accepting: bool) -> State {
        State {
            accepting: is_accepting,
            transition_map: HashMap::new(),
        }
    }

    pub fn add_transition_for_symbol(&mut self, symbol: &str, new_state: Rc<RefCell<State>>) {
        self.transition_map
            .entry(symbol.to_string())
            .or_insert_with(Vec::new)
            .push(new_state);
    }

    pub fn get_transition_for_symbol(&self, symbol: &str) -> Vec<Rc<RefCell<State>>> {
        match self.transition_map.get(symbol) {
            Some(states) => states.clone(),
            None => Vec::new(),
        }
    }
}
#[cfg(test)]
mod test {

    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_add_and_get_transition() {
        let s1 = Rc::new(RefCell::new(State::new(false)));
        let s2 = Rc::new(RefCell::new(State::new(true)));

        s1.borrow_mut()
            .add_transition_for_symbol(EPSILON, s2.clone());
        let transition_table_for_epsilon = s1.borrow().get_transition_for_symbol(EPSILON);

        let first_state = transition_table_for_epsilon.get(0);
        match first_state {
            Some(state) => {
                assert_eq!(*state.borrow(), *s2.borrow());
                assert_eq!(s2.borrow().accepting, true);
            }
            None => {
                panic!("No state found in transition table");
            }
        }
    }

    #[test]
    fn test_multiple_transitions() {
        let s1 = Rc::new(RefCell::new(State::new(false)));
        let s2 = Rc::new(RefCell::new(State::new(false)));
        let s3 = Rc::new(RefCell::new(State::new(true)));

        s1.borrow_mut()
            .add_transition_for_symbol(EPSILON, s2.clone());

        let transition_table_for_epsilon = s1.borrow().get_transition_for_symbol(EPSILON);
        assert_eq!(transition_table_for_epsilon.len(), 1);

        let first_state = transition_table_for_epsilon.get(0);
        match first_state {
            Some(state) => {
                assert_eq!(state.borrow().accepting, false);
            }
            None => {
                panic!("No state found in transition table");
            }
        }

        s2.borrow_mut()
            .add_transition_for_symbol(EPSILON, s3.clone());

        let transition_table_for_s2_epsilon = s2.borrow().get_transition_for_symbol(EPSILON);
        assert_eq!(transition_table_for_s2_epsilon.len(), 1);

        let second_state = transition_table_for_s2_epsilon.get(0);
        match second_state {
            Some(state) => {
                assert_eq!(state.borrow().accepting, true);
            }
            None => {
                panic!("No state found in transition table");
            }
        }
    }
}
