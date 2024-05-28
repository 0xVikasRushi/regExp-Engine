use std::collections::HashMap;

pub const EPSILON: &str = "ε";

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub accepting: bool,
    pub transition_map: HashMap<String, Vec<State>>,
}

impl State {
    pub fn new(is_accepting: bool) -> State {
        State {
            accepting: is_accepting,
            transition_map: HashMap::new(),
        }
    }

    pub fn add_transition_for_symbol(&mut self, symbol: &str, new_state: State) {
        self.transition_map
            .entry(symbol.to_string())
            .or_insert_with(Vec::new)
            .push(new_state);
    }

    pub fn get_transition_for_symbol(&self, symbol: &str) -> Vec<State> {
        match self.transition_map.get(symbol) {
            Some(states) => states.clone(),
            None => Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_add_and_get_transition() {
        let mut s1 = State::new(false);
        let s2 = State::new(true);

        s1.add_transition_for_symbol(EPSILON, s2.clone());
        let transition_table_for_a = &s1.get_transition_for_symbol(EPSILON);

        let first_state = transition_table_for_a.get(0);
        match first_state {
            Some(state) => {
                assert_eq!(&s2, state);
                assert_eq!(s2.accepting, true);
            }
            None => {
                panic!("No state found in transition table");
            }
        }
    }

    #[test]
    fn test_multiple_transitions() {
        let mut s1 = State::new(false);
        let mut s2 = State::new(false);
        let s3 = State::new(true);

        s1.add_transition_for_symbol(EPSILON, s2.clone());

        let transition_table_for_a = s1.get_transition_for_symbol(EPSILON);
        assert_eq!(transition_table_for_a.len(), 1);

        let first_state = transition_table_for_a.get(0);
        match first_state {
            Some(state) => {
                assert_eq!(state.accepting, false);
            }
            None => {
                panic!("No state found in transition table");
            }
        }
        s2.add_transition_for_symbol(EPSILON, s3.clone());

        let transition_table_for_b = &s2.get_transition_for_symbol(EPSILON);

        assert_eq!(transition_table_for_b.len(), 1);

        let second_state = transition_table_for_b.get(0);
        match second_state {
            Some(state) => {
                assert_eq!(state.accepting, true);
            }
            None => {
                panic!("No state found in transition table");
            }
        }
    }
}
