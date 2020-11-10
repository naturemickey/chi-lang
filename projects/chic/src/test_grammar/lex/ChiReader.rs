pub struct ChiReader {
    // s: String,
    // nfa: Rc<NFA>,
    chars: Vec<char>,
    reader_state: ChiReaderState,
}

impl ChiReader {
    pub fn new(nfa: Rc<NFA>, s: String) -> ChiReader {
        let mut chars = Vec::new();

        for c in s.chars() {
            chars.push(c);
        }
        let reader_state = ChiReaderState::new(nfa.clone());

        ChiReader { /*s, nfa,*/ chars, reader_state }
    }

    pub fn the_rest_of_tokens(&mut self) -> Vec<Token> {
        let mut res = Vec::new();

        loop {
            match self.get_next_token() {
                Some(token) => res.push(token),
                None => break,
            }
        }

        res
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        let mut index_now = self.reader_state.index_from;

        if index_now >= self.chars.len() {
            return None;
        }

        loop {
            if index_now >= self.chars.len() {
                break;
            }
            let states = self.reader_state.current_states.jump(self.chars[index_now]);
            index_now += 1;

            // println!("states length : {}", states.states.len());

            if states.is_empty() {
                break;
            }


            let accepted_states = states.accepted_states();
            self.reader_state.current_states = states;

            if !accepted_states.is_empty() {
                self.reader_state.index_finish = index_now;
                self.reader_state.last_accepted_states = accepted_states;
            }
        }

        let res = Some(self.build_token());

        self.reader_state.index_from = index_now + 1;
        self.reader_state.current_states = self.reader_state.start_states.clone();
        self.reader_state.last_accepted_states = StateSet::new(vec![]);

        res
    }

    fn build_token(&self) -> Token {
        let accepted_states = &self.reader_state.last_accepted_states;

        if accepted_states.is_empty() {
            panic!("{:?}\n{}", self.chars, self.reader_state.to_string());
        }

        let token_type = (*accepted_states.states[0]).borrow().token_type.clone().unwrap();
        let mut literal = "".to_string();
        for idx in self.reader_state.index_from..self.reader_state.index_finish {
            literal.push(self.chars[idx]);
        }
        Token { token_type: token_type, literal }
    }
}
