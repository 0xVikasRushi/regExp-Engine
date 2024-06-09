use crate::{
    nfa::{self, CELL, NFA},
    state::{State, EPSILON},
};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};
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

    pub fn get_epsilon_and_unique_transitions(
        transition_map: &HashMap<Uuid, Vec<CELL>>,
    ) -> (Vec<Vec<String>>, HashSet<String>) {
        let mut epsilon_transitions: Vec<Vec<String>> = Vec::new();
        let mut unique_transitions: HashSet<String> = HashSet::new();

        for (_, cells) in transition_map {
            let mut epsilon_transition: Vec<String> = Vec::new();

            for cell in cells {
                if cell.symbol == EPSILON {
                    epsilon_transition.extend(cell.transition.iter().cloned());
                } else {
                    unique_transitions.insert(cell.symbol.clone());
                }
            }

            if !epsilon_transition.is_empty() {
                epsilon_transitions.push(epsilon_transition);
            }
        }

        (epsilon_transitions, unique_transitions)
    }

    // ! WIP DFA
    pub fn get_transition_table(
        nfa_table: HashMap<Uuid, Vec<CELL>>,
        accepting_state: Uuid,
        all_e_transitions: Vec<CELL>,
    ) -> (HashMap<Vec<Uuid>, Vec<CELL>>, Vec<Uuid>) {
        let mut dfa_table: HashMap<Vec<Uuid>, Vec<CELL>> = HashMap::new();
        let mut accepting_states: Vec<Uuid> = Vec::new();

        let (epsilon_transitions, unique_transitions) =
            DFA::get_epsilon_and_unique_transitions(&nfa_table);

        let uuid_epsilon_transitions = convert_to_uuid(&epsilon_transitions);

        for transition in uuid_epsilon_transitions {
            if transition.len() >= 2 {
                let mut cells: Vec<CELL> = Vec::new();
                for symbol in unique_transitions.iter() {
                    let new_cell = CELL::new(&symbol);
                    cells.push(new_cell);
                }
                dfa_table.insert(transition, cells);
            }
        }

        (dfa_table, accepting_states)
    }

    pub fn test(_string: &str) -> bool {
        return false;
    }

    pub fn print_dfa_transition_table(dfa_table: &HashMap<Vec<Uuid>, Vec<CELL>>) {
        println!("{:<36} {:<10} {:<10}", "State", "Symbol", "Transitions");
        println!("{:-<60}", "-");

        for (state, cells) in dfa_table {
            let state_str = state
                .iter()
                .map(|uuid| uuid.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            for cell in cells {
                let transitions_str = cell
                    .transition
                    .iter()
                    .map(|uuid| uuid.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                println!(
                    "{:<36} {:<10} {:<10}",
                    state_str, cell.symbol, transitions_str
                );
            }
        }
    }
    pub fn get_accepting_states() {}
}
// WIP Function for dfa minimization which takes as an input dfa_table
// and accepting state
    pub fn dfa_min(
dfa_table: HashMap<Vec<Uuid>, Vec<CELL>>,
accepting_state: Vec<Uuid>
) -> (HashMap<Vec<Uuid>, Vec<CELL>>) {
        // ***
    let dfa_table:
    let accepting_state:
    }


#[cfg(test)]
mod test {
    use crate::{dfa::DFA, nfa::NFA};

    #[test]
    fn test_get_dfa_transition_table() {
        let mut first_nfa = NFA::char("a");
        let mut second_nfa = NFA::char("b");

        let final_nfa = NFA::or_pair(&mut first_nfa, &mut second_nfa);

        let nfa_table = final_nfa.get_transition_table();
        let dfa_table = DFA::get_transition_table(nfa_table.0, nfa_table.1, nfa_table.2);
        DFA::print_dfa_transition_table(&dfa_table.0);
        // println!("Accepting States: {:?}", accepting_states);
    }
}

fn convert_to_uuid(input: &Vec<Vec<String>>) -> Vec<Vec<Uuid>> {
    input
        .into_iter()
        .map(|inner_vec| {
            inner_vec
                .into_iter()
                .map(|s| Uuid::parse_str(&s).expect("Failed to parse UUID"))
                .collect()
        })
        .collect()
}
