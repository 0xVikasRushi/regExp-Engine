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
    ) -> (HashMap<Uuid, Vec<Uuid>>, HashSet<String>) {
        let mut epsilon_transitions: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        let mut unique_transitions: HashSet<String> = HashSet::new();

        for (state, cells) in transition_map {
            let mut epsilon_transition: Vec<Uuid> = vec![*state];

            for cell in cells {
                if cell.symbol == EPSILON {
                    epsilon_transition.extend(cell.transition.iter().cloned());
                } else {
                    unique_transitions.insert(cell.symbol.clone());
                }
            }

            epsilon_transitions.insert(*state, epsilon_transition);
        }

        (epsilon_transitions, unique_transitions)
    }

    // ! WIP DFA
    pub fn get_transition_table(
        nfa_table: HashMap<Uuid, Vec<CELL>>,
        accepting_state: Uuid,
        _all_e_transitions: Vec<CELL>,
    ) -> (HashMap<Vec<Uuid>, Vec<CELL>>, Vec<Vec<Uuid>>) {
        let mut dfa_table: HashMap<Vec<Uuid>, Vec<CELL>> = HashMap::new();
        let mut accepting_states: Vec<Vec<Uuid>> = Vec::new();
        let mut state_queue: Vec<Vec<Uuid>> = Vec::new();
        let mut visited_states: HashSet<Vec<Uuid>> = HashSet::new();

        let (epsilon_transitions, unique_transitions) =
            DFA::get_epsilon_and_unique_transitions(&nfa_table);

        let initial_state = epsilon_transitions
            .get(&accepting_state)
            .cloned()
            .unwrap_or_default();
        state_queue.push(initial_state.clone());
        visited_states.insert(initial_state.clone());

        while let Some(current_state) = state_queue.pop() {
            let mut state_transitions: HashMap<String, Vec<Uuid>> = HashMap::new();

            for uuid in &current_state {
                if let Some(cells) = nfa_table.get(uuid) {
                    for cell in cells {
                        if cell.symbol != EPSILON {
                            state_transitions
                                .entry(cell.symbol.clone())
                                .or_insert_with(Vec::new)
                                .extend(cell.transition.iter().cloned());
                        }
                    }
                }
            }

            let mut dfa_cells: Vec<CELL> = Vec::new();

            for (symbol, mut transitions) in state_transitions {
                let mut epsilon_closure: HashSet<Uuid> = HashSet::new();
                for state in &transitions {
                    if let Some(epsilon_states) = epsilon_transitions.get(state) {
                        epsilon_closure.extend(epsilon_states.iter().cloned());
                    }
                }

                let epsilon_closure_vec: Vec<Uuid> = epsilon_closure.into_iter().collect();

                transitions.sort();
                transitions.dedup();

                let mut new_cell = CELL::new(&symbol);
                new_cell.transition = epsilon_closure_vec.clone();
                dfa_cells.push(new_cell);

                if !visited_states.contains(&epsilon_closure_vec) {
                    state_queue.push(epsilon_closure_vec.clone());
                    visited_states.insert(epsilon_closure_vec.clone());
                }

                if epsilon_closure_vec.contains(&accepting_state) {
                    accepting_states.push(current_state.clone());
                }
            }

            dfa_table.insert(current_state.clone(), dfa_cells);
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

    pub fn get_accepting_states(
        dfa_table: &HashMap<Vec<Uuid>, Vec<CELL>>,
        accepting_state: Uuid,
    ) -> Vec<Vec<Uuid>> {
        dfa_table
            .keys()
            .filter(|state| state.contains(&accepting_state))
            .cloned()
            .collect()
    }
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
