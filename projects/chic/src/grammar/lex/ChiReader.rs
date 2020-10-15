pub struct ChiReader {
    s: String,
    nfa: Rc<NFA>,
    chars: Vec<char>,
    reader_state: ChiReaderState,
}

impl ChiReader {
    pub fn new(nfa: Rc<NFA>, mut s: String) -> ChiReader {
        let mut chars = Vec::new();

        loop {
            match s.pop() {
                Some(c) => chars.push(c),
                None => break,
            }
        }
        let reader_state = ChiReaderState::new(nfa.clone());

        ChiReader { s, nfa, chars, reader_state }
    }

    pub fn get_next_token(&mut self) -> Token {
        let index_now = self.reader_state.index_now;

        let states = self.reader_state.current_states.jump(self.chars[index_now]);

        let accepted_states = states.acepted_states();

        todo!()
    }
}

struct ChiReaderState {
    current_states: StateSet,
    last_accepted_states: StateSet,
    index_from: usize,
    index_finish: usize,
    index_now: usize,
}

impl ChiReaderState {
    pub fn new(nfa: Rc<NFA>) -> ChiReaderState {
        let mut current_states = StateSet::new(vec![nfa.start.clone()]);

        for s in (*nfa.start).borrow().eq_state_vec() {
            current_states.add(s.clone());
        }

        let last_accepted_states = StateSet { states: Vec::new() };

        Self { current_states, last_accepted_states, index_from: 0, index_finish: 0, index_now: 0 }
    }
}