use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use uuid::Uuid;

pub const EPSILON: &str = "ε";
pub const EPSILON_STAR: &str = "ε*";
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

    pub fn get_all_transition_symbols(&self) -> Vec<String> {
        self.transition_map.keys().cloned().collect()
    }

    // ? DFA Traversal

    pub fn count_unique_transitions(
        &self,
    ) -> (u64, HashSet<String>, HashSet<Uuid>, HashMap<Uuid, State>) {
        let mut stack: Vec<Rc<RefCell<State>>> = Vec::new();
        let mut is_visited: HashMap<Uuid, bool> = HashMap::new();
        let mut count: u64 = 0;

        let mut all_transition_symbols: HashSet<String> = HashSet::new();
        let mut all_uuid: HashSet<Uuid> = HashSet::new();
        let mut map: HashMap<Uuid, State> = HashMap::new();

        stack.push(Rc::new(RefCell::new(self.clone())));

        while let Some(curr_state) = stack.pop() {
            let curr_state_ref = curr_state.borrow();
            let curr_label = curr_state_ref.label;
            all_uuid.insert(curr_label);
            map.insert(curr_label, curr_state_ref.clone());

            if is_visited.get(&curr_state_ref.label) == Some(&true) {
                continue;
            }

            count += 1;
            is_visited.insert(curr_state_ref.label.clone(), true);

            let all_transition = curr_state_ref.get_all_transition_symbols();
            for next_transition in all_transition {
                all_transition_symbols.insert(next_transition.clone());
                let next_states = curr_state_ref.get_transition_for_symbol(&next_transition);
                for next_state in next_states {
                    if is_visited.get(&next_state.borrow().label) != Some(&true) {
                        stack.push(next_state.clone());
                    }
                }
            }
        }

        return (count, all_transition_symbols, all_uuid, map);
    }

    // EPSILON
    pub fn epslion_closure(&self) -> Vec<State> {
        let mut epsilon_vector: Vec<State> = Vec::new();

        epsilon_vector.push(self.clone());

        let all_symbols = self.get_all_transition_symbols();

        for symbol in all_symbols.iter() {
            let states = self.get_transition_for_symbol(symbol);

            for state in states.iter() {
                let curr_state = state.borrow().clone();
                epsilon_vector.push(curr_state);
            }
        }
        return epsilon_vector;
    }

    pub fn test(&self, _string: &str) -> bool {
        return self.test_helper(_string, HashMap::new());
    }

    pub fn test_helper(&self, _string: &str, mut is_visited: HashMap<Uuid, bool>) -> bool {
        let label = self.label;

        let is_curr_visited = is_visited.get(&label);

        match is_curr_visited {
            Some(val) => {
                if *val == true {
                    return false;
                }
            }
            None => {}
        }

        is_visited.insert(label, true);

        if _string.is_empty() {
            if self.accepting {
                return true;
            }

            let epsilon_transitions = self.get_transition_for_symbol(EPSILON);

            for next_state in epsilon_transitions.iter() {
                if next_state.borrow().test_helper("", is_visited.clone()) {
                    return true;
                }
            }
            return false;
        }

        let first_char = _string.chars().next().unwrap().to_string();
        let rest_of_string = &_string[first_char.len()..];

        let symbol_transitions = self.get_transition_for_symbol(&first_char);

        for next_state in symbol_transitions.iter() {
            if next_state
                .borrow()
                .test_helper(rest_of_string, is_visited.clone())
            {
                return true;
            }
        }

        let eplision_transition_for_next_state = self.get_transition_for_symbol(EPSILON);

        for next_state in eplision_transition_for_next_state.iter() {
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
    fn test_count_unique_transitions() {
        let mut nfa_1 = NFA::char("a");
        let mut nfa_2 = NFA::char("b");

        let final_concat_nfa = NFA::concat_pair(&mut nfa_1, &mut nfa_2);
        let res = final_concat_nfa
            .in_state
            .borrow()
            .count_unique_transitions();

        let concat_count = res.0;
        assert_eq!(concat_count, 4);

        assert_eq!(res.2.len(), concat_count.try_into().unwrap());

        let map = res.1;
        assert!(map.contains("a"));
        assert!(map.contains("b"));
        assert!(map.contains(EPSILON));
        assert!(!map.contains("suii"));

        let mut nfa_3 = NFA::char("c");
        let mut nfa_4 = NFA::char("d");

        let or_nfa = NFA::or_pair(&mut nfa_3, &mut nfa_4);
        let or_pair_count = or_nfa.in_state.borrow().count_unique_transitions();

        let map = or_pair_count.1;
        assert_eq!(or_pair_count.0, 6);
        assert!(map.contains("c"));
        assert!(map.contains("d"));
        assert!(map.contains(EPSILON));
    }

    #[test]
    fn test_get_all_transition_symbols() {
        let s1 = Rc::new(RefCell::new(State::new(false)));
        let s2 = Rc::new(RefCell::new(State::new(true)));

        s1.borrow_mut()
            .add_transition_for_symbol(EPSILON, s2.clone());
        s1.borrow_mut().add_transition_for_symbol("a", s2.clone());

        s1.borrow_mut().add_transition_for_symbol("b", s2.clone());
        s1.borrow_mut().add_transition_for_symbol("c", s2.clone());
        s1.borrow_mut().add_transition_for_symbol("j", s2.clone());
        s1.borrow_mut().add_transition_for_symbol("d", s2.clone());

        let mut sui = s1.borrow().get_all_transition_symbols();
        sui.sort();

        assert_eq!(sui.len(), 6);
        assert_eq!(sui[0], "a");
        assert_eq!(sui[1], "b");
        assert_eq!(sui[2], "c");
        assert_eq!(sui[3], "d");
        assert_eq!(sui[4], "j");
        assert_eq!(sui[5], EPSILON);
    }

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

    fn test_epslion_closure() {
        let mut first_nfa = NFA::char("a");
        let mut second_nfa = NFA::char("b");
        let final_nfa = NFA::or_pair(&mut first_nfa, &mut second_nfa);

        let epsilon_closure = final_nfa.in_state.borrow().epslion_closure();

        assert_eq!(epsilon_closure.len(), 3);
        assert_eq!(epsilon_closure[0].label, final_nfa.in_state.borrow().label);
        assert_eq!(epsilon_closure[1].label, first_nfa.in_state.borrow().label);
        assert_eq!(epsilon_closure[2].label, second_nfa.in_state.borrow().label);
    }
}
