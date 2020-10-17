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
        let mut index_now = self.reader_state.index_from;

        loop {
            let states = self.reader_state.current_states.jump(self.chars[index_now]);
            index_now += 1;
            if states.is_empty() {
                break;
            }

            let accepted_states = states.accepted_states();
            if !accepted_states.token_types().is_empty() {
                self.reader_state.index_finish = index_now;
            }
        }

        self.build_token()
    }

    fn build_token(&self) -> Token {
        let accepted_states = self.reader_state.last_accepted_states.accepted_states();
        let token_type = (*accepted_states.states[0]).borrow().token_type.clone().unwrap();
        let mut literal = "".to_string();
        for idx in self.reader_state.index_from..self.reader_state.index_finish {
            literal.push(self.chars[idx]);
        }
        Token { token_type: token_type, literal }
    }
}

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