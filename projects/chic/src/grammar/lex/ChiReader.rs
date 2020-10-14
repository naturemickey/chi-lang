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

    pub fn get_token(&mut self) -> Token {
        todo!()
    }
}

struct ChiReaderState {
    current_states: HashSet<Rc<State>>,
    last_token: Token,
    last_accepted_states: HashSet<Rc<State>>,
}

impl ChiReaderState {
    pub fn new(nfa:Rc<NFA>) -> ChiReaderState {
        let mut current_states = HashSet::new();
        let _nfa:&NFA = nfa.borrow();
        current_states.insert(_nfa.start.clone());
        todo!()
    }
}