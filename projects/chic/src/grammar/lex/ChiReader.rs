pub struct ChiReader {
    nfa: Rc<NFA>,
    tokens: Vec<Token>,
}

impl ChiReader {
    pub fn new(nfa: Rc<NFA>, chars: Chars) -> ChiReader {
        todo!()
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