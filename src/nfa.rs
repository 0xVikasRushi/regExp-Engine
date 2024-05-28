use crate::state::{State, EPSILON};

#[derive(Clone)]
pub struct NFA {
    pub in_state: State,
    pub out_state: State,
}
impl NFA {
    pub fn new() -> NFA {
        NFA {
            in_state: State::new(false),
            out_state: State::new(true),
        }
    }
    pub fn test() -> bool {
        false
    }

    pub fn char(symbol: &str) -> NFA {
        let mut nfa = NFA::new();
        nfa.out_state.accepting = true;

        nfa.in_state
            .add_transition_for_symbol(symbol, nfa.out_state.clone());

        return nfa;
    }

    pub fn concat_pair(first: &mut NFA, second: &mut NFA) -> NFA {
        first.out_state.accepting = false;
        second.out_state.accepting = true;

        first
            .out_state
            .add_transition_for_symbol(EPSILON, second.in_state.clone());

        NFA {
            in_state: first.in_state.clone(),
            out_state: second.out_state.clone(),
        }
    }

    pub fn concat(first: &mut NFA, array_of_nfa: &mut Vec<NFA>) -> NFA {
        let mut current_nfa = first.clone();

        for nfa in array_of_nfa.iter_mut() {
            current_nfa = NFA::concat_pair(&mut current_nfa, nfa);
        }

        current_nfa
    }
}
#[cfg(test)]
mod test {

    use super::*;
    use crate::state::EPSILON;

    #[test]
    fn test_concat() {
        let mut first = NFA::char("a");
        let second = NFA::char("b");
        let third = NFA::char("c");

        let mut array_of_nfa = vec![second, third];

        let final_nfa = NFA::concat(&mut first, &mut array_of_nfa);

        assert_eq!(final_nfa.in_state.accepting, false);
        assert_eq!(final_nfa.out_state.accepting, true);

        let first_in_state = &final_nfa.in_state;
        let first_out_state = &final_nfa.in_state;

        assert_eq!(*first_in_state, first.in_state);
        assert_eq!(*first_out_state, first.out_state);
    }

    #[test]
    fn test_concat_pair() {
        let mut first = NFA::char("a");
        let mut second = NFA::char("b");

        let combine_transition = NFA::concat_pair(&mut first, &mut second);

        assert_eq!(first.in_state.accepting, false);
        assert_eq!(first.out_state.accepting, false);

        assert_eq!(second.in_state.accepting, false);
        assert_eq!(second.out_state.accepting, true);

        assert_eq!(combine_transition.in_state.accepting, false);
        assert_eq!(combine_transition.out_state.accepting, true);

        let epslion_transition = first.out_state.get_transition_for_symbol(EPSILON);
        assert_eq!(epslion_transition.len(), 1);

        let epslion_transition_state = epslion_transition.get(0).unwrap();
        assert_eq!(*epslion_transition_state, second.in_state);

        let b_transition = second.in_state.get_transition_for_symbol("b");
        assert_eq!(b_transition.len(), 1);

        let b_expected_out_state = b_transition.get(0).unwrap();
        assert_eq!(*b_expected_out_state, second.out_state);
    }
}
