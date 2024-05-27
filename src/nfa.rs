use crate::state::State;

pub const EPSILON: &str = "ε";

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

        nfa.in_state
            .add_transition_for_symbol(symbol, nfa.out_state.clone());

        return nfa;
    }
    pub fn concat_pair(mut first: NFA, mut second: NFA) -> NFA {
        first.out_state.accepting = false;
        second.out_state.accepting = true;

        first
            .out_state
            .add_transition_for_symbol(EPSILON, second.in_state);

        NFA {
            in_state: first.in_state,
            out_state: second.out_state,
        }
    }

    pub fn concat(mut first: NFA, array_of_nfa: Vec<NFA>) -> NFA {
        NFA::new()
    }
}
