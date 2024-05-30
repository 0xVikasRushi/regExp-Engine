# Regular Expression in Finite State Machine

This project aims to develop a regex matching engine in Rust that converts regular expressions into a Non-deterministic Finite Automaton (NFA), then transforms the NFA into a Deterministic Finite Automaton (DFA), and finally uses the DFA to determine whether a given string matches the regular expression.

why implementing own regex engine ?

> While open-source libraries exist for regex matching, the purpose here is just educational, focusing on the underlying of finite state machines.

> **Note:** It is not recommended to use this project in production unless you want to trust a 21-year-old.

## Project Milestones

1. Implementing NFA State Machine

   - Simple State Construction
   - State Traditions
   - Elision Transition
   - Concatenation Machine
   - Union Machine
   - Kleene Closure - A\*

2. Converting NFA -> DFA Machine
   - NFA Acceptor
   - Building DFA Table
   - DFA Minimization
   - RegExp Match
