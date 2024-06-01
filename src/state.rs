use std::{cell::RefCell, collections::HashMap, rc::Rc};

use uuid::Uuid;

pub const EPSILON: &str = "ε";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub accepting: bool,
    pub transition_map: HashMap<String, Vec<Rc<RefCell<State>>>>,
    pub label: Uuid,
}

impl State {
    pub fn new(is_accepting: bool) -> State {
        State {
            accepting: is_accepting,
            transition_map: HashMap::new(),
            label: Uuid::new_v4(),
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

    pub fn test(&self, _string: &str) -> bool {
        return self.test_helper(_string, HashMap::new());
    }

    pub fn test_helper(&self, _string: &str, mut is_visited: HashMap<Uuid, bool>) -> bool {
        let label = &self.label;

        let is_curr_visited = is_visited.get(label);

        match is_curr_visited {
            Some(val) => {
                if *val == true {
                    return false;
                }
            }
            None => {
                is_visited.insert(*label, true);
            }
        }

        if _string.is_empty() {
            if self.accepting {
                return true;
            }

            let mut epsilon_transitions = self.get_transition_for_symbol(EPSILON);

            for next_state in epsilon_transitions.iter_mut() {
                if next_state.borrow().test_helper("", is_visited.clone()) {
                    return true;
                }
            }
            return false;
        }

        let first_char = _string.chars().next().unwrap().to_string();
        let rest_of_string = &_string[first_char.len()..];

        let mut symbol_transitions = self.get_transition_for_symbol(&first_char);

        for next_state in symbol_transitions.iter_mut() {
            if next_state
                .borrow()
                .test_helper(rest_of_string, is_visited.clone())
            {
                return true;
            }
        }

        let mut eplision_transition_for_next_state = self.get_transition_for_symbol(EPSILON);

        for next_state in eplision_transition_for_next_state.iter_mut() {
            if next_state.borrow().test_helper(_string, is_visited.clone()) {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod test {

    use crate::nfa::NFA;

    use crate::state::State;
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::EPSILON;

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

    #[test]
    fn test_regex_concat() {
        let first_nfa = NFA::char("a");
        let second_nfa = NFA::char("b");
        let final_nfa = NFA::concat(&first_nfa, &[second_nfa]);

        let result_1 = final_nfa.test("ab");
        let result_2 = final_nfa.test("ac");
        let result_3 = final_nfa.test(EPSILON);
        let result_4 = final_nfa.test(" ab");
        let result_5 = final_nfa.test("ab ");

        assert_eq!(result_1, true);
        assert_eq!(result_2, false);
        assert_eq!(result_3, false);
        assert_eq!(result_4, false);
        assert_eq!(result_5, false);
    }
    #[test]
    fn test_regex_rep() {
        let mut first_nfa = NFA::char("a");

        let final_nfa = NFA::rep(&mut first_nfa);

        let result_1 = final_nfa.test("a");
        let result_2 = final_nfa.test("aa");
        let result_3 = final_nfa.test("aaa");
        let result_4 = final_nfa.test("aaaa");
        let result_5 = final_nfa.test("");
        let result_6 = final_nfa.test(" ");

        assert_eq!(result_1, true);
        assert_eq!(result_2, true);
        assert_eq!(result_3, true);
        assert_eq!(result_4, true);
        assert_eq!(result_5, true);
        assert_eq!(result_6, false);
    }

    #[test]
    // ! RegExp /xy*|z/
    fn test_regex_or_and_concat() {
        let mut x = NFA::char("x");
        let y_star = NFA::rep(&mut NFA::char("y"));
        let mut z = NFA::char("z");

        let final_nfa = NFA::or_pair(&mut NFA::concat(&mut x, &[y_star]), &mut z);
        let result_1 = final_nfa.test("x");
        let result_2 = final_nfa.test("xy");
        let result_3 = final_nfa.test("xyy");
        let result_4 = final_nfa.test("z");
        let result_5 = final_nfa.test("a");
        let result_6 = final_nfa.test("");

        assert_eq!(result_1, true);
        assert_eq!(result_2, true);
        assert_eq!(result_3, true);
        assert_eq!(result_4, true);
        assert_eq!(result_5, false);
        assert_eq!(result_6, false);
    }
}
