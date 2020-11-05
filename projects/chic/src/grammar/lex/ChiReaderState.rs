
struct ChiReaderState {
    current_states: StateSet,
    last_accepted_states: StateSet,
    index_from: usize,
    index_finish: usize,
}

impl ChiReaderState {
    pub fn new(nfa: Rc<NFA>) -> ChiReaderState {
        let mut current_states = StateSet::new(vec![nfa.start.clone()]);

        // 添加所有等价节点
        current_states.merge((*nfa.start).borrow().eq_state_vec());

        let last_accepted_states = StateSet::new(vec![]);

        Self { current_states, last_accepted_states, index_from: 0, index_finish: 0 }
    }
}