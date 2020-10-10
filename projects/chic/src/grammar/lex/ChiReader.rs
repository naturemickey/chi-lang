pub struct ChiReader {
    nfa: Rc<NFA>,
    tokens: Vec<Token>,
}

impl ChiReader {
    pub fn new(nfa: Rc<NFA>, chars: Chars) -> ChiReader {
        panic!()
    }
}

struct ChiReaderState {
    current_states: Vec<Rc<State>>,
    last_token: Token,
    last_accepted_states: Vec<Rc<State>>,
}