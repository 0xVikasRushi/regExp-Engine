use uuid::Uuid;

use crate::state::{State, EPSILON};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct NFA {
    pub in_state: Rc<RefCell<State>>,
    pub out_state: Rc<RefCell<State>>,
}

impl NFA {
    pub fn new() -> NFA {
        NFA {
            in_state: Rc::new(RefCell::new(State::new(false))),
            out_state: Rc::new(RefCell::new(State::new(true))),
        }
    }

    pub fn char(symbol: &str) -> NFA {
        let nfa = NFA::new();
        nfa.out_state.borrow_mut().accepting = true;
        nfa.in_state
            .borrow_mut()
            .add_transition_for_symbol(symbol, nfa.out_state.clone());
        nfa
    }

    pub fn add_epsilon_transition(&mut self) {
        self.in_state
            .borrow_mut()
            .add_transition_for_symbol(EPSILON, self.out_state.clone());
    }

    pub fn test(&self, _string: &str) -> bool {
        self.in_state.borrow().test(_string)
    }

    pub fn concat_pair(first: &mut NFA, second: &mut NFA) -> NFA {
        first.out_state.borrow_mut().accepting = false;
        second.out_state.borrow_mut().accepting = true;

        first
            .out_state
            .borrow_mut()
            .add_transition_for_symbol(EPSILON, second.in_state.clone());

        NFA {
            in_state: first.in_state.clone(),
            out_state: second.out_state.clone(),
        }
    }

    pub fn concat(first: &NFA, array_of_nfa: &[NFA]) -> NFA {
        let mut current_nfa = first.clone();

        for nfa in array_of_nfa.iter() {
            current_nfa = NFA::concat_pair(&mut current_nfa, &mut nfa.clone());
        }

        current_nfa
    }

    pub fn or_pair(first: &mut NFA, second: &mut NFA) -> NFA {
        let final_nfa = NFA::new();

        final_nfa
            .in_state
            .borrow_mut()
            .add_transition_for_symbol(EPSILON, first.in_state.clone());

        final_nfa
            .in_state
            .borrow_mut()
            .add_transition_for_symbol(EPSILON, second.in_state.clone());

        first.out_state.borrow_mut().accepting = false;
        second.out_state.borrow_mut().accepting = false;

        first
            .out_state
            .borrow_mut()
            .add_transition_for_symbol(EPSILON, final_nfa.out_state.clone());

        second
            .out_state
            .borrow_mut()
            .add_transition_for_symbol(EPSILON, final_nfa.out_state.clone());

        final_nfa
    }

    pub fn rep(first: &mut NFA) -> NFA {
        let final_nfa = NFA::new();

        final_nfa
            .in_state
            .borrow_mut()
            .add_transition_for_symbol(EPSILON, first.in_state.clone());

        final_nfa
            .in_state
            .borrow_mut()
            .add_transition_for_symbol(EPSILON, final_nfa.out_state.clone());

        first.out_state.borrow_mut().accepting = false;

        first
            .out_state
            .borrow_mut()
            .add_transition_for_symbol(EPSILON, final_nfa.out_state.clone());

        final_nfa
            .out_state
            .borrow_mut()
            .add_transition_for_symbol(EPSILON, first.in_state.clone());

        return final_nfa;
    }

    pub fn get_transition_table(&self) -> i32 {
        let mut stack: Vec<State> = Vec::new();
        let mut is_visited: HashMap<Uuid, bool> = HashMap::new();

        stack.push(self.in_state.borrow().clone());

        let mut count = 0;

        while let Some(curr_state) = stack.pop() {
            let visit = is_visited.get(&curr_state.label);
            if visit == Some(&true) {
                continue;
            }

            count += 1;
            is_visited.insert(curr_state.label, true);

            let all_transitions = curr_state.get_all_transition_symbols();

            for next_transition in all_transitions.iter() {
                let next_states = curr_state.get_transition_for_symbol(next_transition);
                for next_state in next_states {
                    if is_visited.get(&next_state.borrow().label) != Some(&true) {
                        let state = next_state.borrow().clone();
                        stack.push(state);
                    }
                }
            }
        }

        return count;
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::state::{State, EPSILON};
    use std::rc::Rc;

    #[test]
    fn test_char() {
        let nfa = NFA::char("a");
        assert_eq!(nfa.in_state.borrow_mut().accepting, false);
        assert_eq!(nfa.out_state.borrow_mut().accepting, true);

        let first_transition = nfa.in_state.borrow_mut().get_transition_for_symbol("a");

        assert!(Rc::ptr_eq(&first_transition[0], &nfa.out_state));
        assert_eq!(first_transition.len(), 1);
    }

    #[test]
    fn test_add_epsilon_transition() {
        let mut nfa = NFA::new();
        nfa.add_epsilon_transition();

        assert_eq!(nfa.in_state.borrow_mut().accepting, false);
        assert_eq!(nfa.out_state.borrow_mut().accepting, true);

        let first_transition = nfa.in_state.borrow_mut().get_transition_for_symbol(EPSILON);

        assert!(Rc::ptr_eq(&first_transition[0], &nfa.out_state));
        assert_eq!(first_transition.len(), 1);
    }

    #[test]
    fn test_concat_pair() {
        let mut first = NFA::char("a");
        let mut second = NFA::char("b");

        let combine_transition = NFA::concat_pair(&mut first, &mut second);

        assert_eq!(first.in_state.borrow_mut().accepting, false);
        assert_eq!(first.out_state.borrow_mut().accepting, false);

        assert_eq!(second.in_state.borrow_mut().accepting, false);
        assert_eq!(second.out_state.borrow_mut().accepting, true);

        assert_eq!(combine_transition.in_state.borrow_mut().accepting, false);
        assert_eq!(combine_transition.out_state.borrow_mut().accepting, true);

        let first_transition = first.in_state.borrow_mut().get_transition_for_symbol("a");

        assert!(Rc::ptr_eq(&first_transition[0], &first.out_state));
        assert_eq!(first_transition.len(), 1);

        let epsilon_transition = first
            .out_state
            .borrow_mut()
            .get_transition_for_symbol(EPSILON);
        assert_eq!(epsilon_transition.len(), 1);

        let second_transition = second.in_state.borrow_mut().get_transition_for_symbol("b");
        assert_eq!(second_transition.len(), 1);

        // ? checking ptr location because RC allows multiple ownership fk rust
        assert!(Rc::ptr_eq(&second_transition[0], &second.out_state));
    }
    #[test]
    fn test_concat() {
        let mut first = NFA::char("a");
        let second = NFA::char("b");
        let third = NFA::char("c");

        let mut array_of_nfa = vec![second.clone(), third.clone()];

        let final_nfa = NFA::concat(&mut first, &mut array_of_nfa);

        assert_eq!(final_nfa.in_state.borrow_mut().accepting, false);
        assert_eq!(final_nfa.out_state.borrow_mut().accepting, true);

        assert_eq!(first.in_state.borrow_mut().accepting, false);
        assert_eq!(first.out_state.borrow_mut().accepting, false);

        assert_eq!(second.in_state.borrow_mut().accepting, false);
        assert_eq!(second.out_state.borrow_mut().accepting, false);

        assert_eq!(third.in_state.borrow_mut().accepting, false);
        assert_eq!(third.out_state.borrow_mut().accepting, true);

        let first_transition = first.in_state.borrow_mut().get_transition_for_symbol("a");
        assert_eq!(first_transition.len(), 1);
        assert!(Rc::ptr_eq(&first_transition[0], &first.out_state));

        let epsilon_transition_first = first
            .out_state
            .borrow_mut()
            .get_transition_for_symbol(EPSILON);
        assert_eq!(epsilon_transition_first.len(), 1);
        assert!(Rc::ptr_eq(&epsilon_transition_first[0], &second.in_state));

        let second_transition = second.in_state.borrow_mut().get_transition_for_symbol("b");
        assert_eq!(second_transition.len(), 1);
        assert!(Rc::ptr_eq(&second_transition[0], &second.out_state));

        let epsilon_transition_second = second
            .out_state
            .borrow_mut()
            .get_transition_for_symbol(EPSILON);
        assert_eq!(epsilon_transition_second.len(), 1);
        assert!(Rc::ptr_eq(&epsilon_transition_second[0], &third.in_state));

        let third_transition = third.in_state.borrow_mut().get_transition_for_symbol("c");
        assert_eq!(third_transition.len(), 1);
        assert!(Rc::ptr_eq(&third_transition[0], &third.out_state));
    }

    #[test]
    fn test_or_pair() {
        let mut first = NFA::char("a");
        let mut second = NFA::char("b");
        let final_nfa = NFA::or_pair(&mut first, &mut second);

        assert_eq!(final_nfa.in_state.borrow().accepting, false);
        assert_eq!(final_nfa.out_state.borrow().accepting, true);

        assert_eq!(first.in_state.borrow().accepting, false);
        assert_eq!(first.out_state.borrow().accepting, false);

        assert_eq!(second.in_state.borrow().accepting, false);
        assert_eq!(second.out_state.borrow().accepting, false);

        let epsilon_transit = final_nfa
            .in_state
            .borrow()
            .get_transition_for_symbol(EPSILON);
        assert_eq!(epsilon_transit.len(), 2);
        assert!(Rc::ptr_eq(&epsilon_transit[0], &first.in_state));
        assert!(Rc::ptr_eq(&epsilon_transit[1], &second.in_state));

        let first_out_transit = first.out_state.borrow().get_transition_for_symbol(EPSILON);
        assert_eq!(first_out_transit.len(), 1);
        assert!(Rc::ptr_eq(&first_out_transit[0], &final_nfa.out_state));

        let second_out_transit = second.out_state.borrow().get_transition_for_symbol(EPSILON);
        assert_eq!(second_out_transit.len(), 1);
        assert!(Rc::ptr_eq(&second_out_transit[0], &final_nfa.out_state));
    }

    #[test]
    fn test_or_rep() {
        let mut a_state_machine = NFA::char("a");

        let final_nfa = NFA::rep(&mut a_state_machine);

        assert_eq!(final_nfa.in_state.borrow_mut().accepting, false);
        assert_eq!(final_nfa.out_state.borrow_mut().accepting, true);

        assert_eq!(a_state_machine.in_state.borrow_mut().accepting, false);
        assert_eq!(a_state_machine.out_state.borrow_mut().accepting, false);

        let first_transitions = final_nfa
            .in_state
            .borrow_mut()
            .get_transition_for_symbol(EPSILON);

        assert_eq!(first_transitions.len(), 2);
        assert!(Rc::ptr_eq(&first_transitions[0], &a_state_machine.in_state));

        // ? final in_state -> final out_state with elision transition
        assert!(Rc::ptr_eq(
            &first_transitions[1],
            &final_nfa.out_state.clone()
        ));

        let second_transition = a_state_machine
            .in_state
            .borrow_mut()
            .get_transition_for_symbol("a");

        assert!(Rc::ptr_eq(
            &second_transition[0],
            &a_state_machine.out_state
        ));
        assert_eq!(second_transition.len(), 1);

        let third_transition = a_state_machine
            .out_state
            .borrow_mut()
            .get_transition_for_symbol(EPSILON);

        assert_eq!(third_transition.len(), 1);
        assert!(Rc::ptr_eq(&third_transition[0], &final_nfa.out_state));

        let fourth_transition = final_nfa
            .out_state
            .borrow_mut()
            .get_transition_for_symbol(EPSILON);

        assert_eq!(fourth_transition.len(), 1);
        assert!(Rc::ptr_eq(&fourth_transition[0], &a_state_machine.in_state));
    }

    #[test]
    fn test_get_transition_table() {
        let mut nfa_1 = NFA::char("a");
        let mut nfa_2 = NFA::char("b");

        let or_machine_nfa = NFA::or_pair(&mut nfa_1, &mut nfa_2);

        let sui = or_machine_nfa.get_transition_table();

        dbg!(sui);
    }
}
