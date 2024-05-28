use crate::state::{State, EPSILON};
use std::cell::RefCell;
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

    pub fn concat(first: &mut NFA, array_of_nfa: &mut Vec[NFA]) -> NFA {
        let mut current_nfa = first;

        for nfa in array_of_nfa.iter() {
            *current_nfa = NFA::concat_pair(current_nfa, nfa);
        }

        current_nfa.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::state::EPSILON;

    #[test]
    fn test_concat_pair() {
        let mut first = NFA::char("a");
        let mut second = NFA::char("b");

        let combine_transition = NFA::concat_pair(&mut first, &mut second);

        assert_eq!(first.in_state.borrow().accepting, false);
        assert_eq!(first.out_state.borrow().accepting, false);

        assert_eq!(second.in_state.borrow().accepting, false);
        assert_eq!(second.out_state.borrow().accepting, true);

        assert_eq!(combine_transition.in_state.borrow().accepting, false);
        assert_eq!(combine_transition.out_state.borrow().accepting, true);

        let epsilon_transition = first.out_state.borrow().get_transition_for_symbol(EPSILON);
        assert_eq!(epsilon_transition.len(), 1);

        let epsilon_transition_state = epsilon_transition.get(0).unwrap();
        assert_eq!(
            *epsilon_transition_state.borrow(),
            *second.in_state.borrow()
        );

        let b_transition = second.in_state.borrow().get_transition_for_symbol("b");
        assert_eq!(b_transition.len(), 1);

        let b_expected_out_state = b_transition.get(0).unwrap();
        assert_eq!(*b_expected_out_state.borrow(), *second.out_state.borrow());
    }

    #[test]
    fn test_concat() {
        let mut first = NFA::char("a");
        let second = NFA::char("b");
        let third = NFA::char("c");

        dbg!(first.clone());
        let mut array_of_nfa = vec![second, third];

        let final_nfa = NFA::concat(&mut first, &mut array_of_nfa);

        assert_eq!(final_nfa.in_state.borrow().accepting, false);
        assert_eq!(final_nfa.out_state.borrow().accepting, true);

        assert_eq!(*final_nfa.in_state.borrow(), *first.in_state.borrow());
        assert_eq!(
            *final_nfa.out_state.borrow(),
            *array_of_nfa.last().unwrap().out_state.borrow()
        );
    }
}
