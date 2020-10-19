
struct ChiReaderState {
    current_states: StateSet,
    last_accepted_states: StateSet,
    index_from: usize,
    index_finish: usize,
}

impl ChiReaderState {
    pub fn new(nfa: Rc<NFA>) -> ChiReaderState {
        let mut current_states = StateSet::new(vec![nfa.start.clone()]);

        for s in (*nfa.start).borrow().eq_state_vec() {
            current_states.add(s.clone());
        }

        let last_accepted_states = StateSet { states: Vec::new() };

        Self { current_states, last_accepted_states, index_from: 0, index_finish: 0 }
    }
}