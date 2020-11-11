struct ChiReaderState {
    start_states: StateSet,
    current_states: StateSet,
    last_accepted_states: StateSet,
    index_from: usize,
    index_finish: usize,
}

impl ChiReaderState {
    pub fn new(nfa: Rc<NFA>) -> ChiReaderState {
        let mut start_states = StateSet::new(vec![nfa.start.clone()]);
        // 添加所有等价节点
        start_states.merge((*nfa.start).borrow().eq_state_vec());

        let current_states = start_states.clone();

        let last_accepted_states = StateSet::new(vec![]);

        Self { start_states, current_states, last_accepted_states, index_from: 0, index_finish: 0 }
    }
}

impl Display for ChiReaderState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{index_from:{}, index_finish:{}}}", self.index_from, self.index_finish)
    }
}