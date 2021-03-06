pub struct ChiReader {
    // s: String,
    // nfa: Rc<NFA>,
    chars: Vec<char>,
    reader_state: ChiReaderState,
}

impl ChiReader {
    pub fn new(nfa: Rc<NFA>, s: &str) -> ChiReader {
        // println!("nfa: {}", nfa);

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
                Some(token) => {
                    match token.skip{
                        true => {}
                        false => res.push(token)
                    }
                }
                None => break,
            }
        }

        res
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        // println!("current_states: {}", self.reader_state.current_states);

        let from = self.reader_state.index_from;

        let mut index_now = self.reader_state.index_from;

        // println!("index_now & chars.len: {}, {}", index_now, self.chars.len());

        if index_now >= self.chars.len() {
            return None;
        }

        loop {
            // println!("index_now & chars.len: {}, {}", index_now, self.chars.len());
            if index_now >= self.chars.len() {
                break;
            }
            let states = self.reader_state.current_states.jump(self.chars[index_now]);

            // println!("states length : {}", states.states.len());
            // println!("states: {}", states);

            if states.is_empty() {
                if self.reader_state.last_accepted_states.is_empty() {
                    panic!("index_now: {}, char: {}, current_states: {}, start_states: {},\n",
                           index_now, self.chars[index_now], self.reader_state.current_states, self.reader_state.start_states);
                }
                break;
            }

            index_now += 1;

            let accepted_states = states.accepted_states();
            self.reader_state.current_states = states;

            // println!("reader_state: {}", self.reader_state);
            // println!("accepted_state_len: {}", accepted_states.len());

            if !accepted_states.is_empty() {
                self.reader_state.index_finish = index_now;
                self.reader_state.last_accepted_states = accepted_states;
            }
        }

        let res = Some(self.build_token());

        self.reader_state.index_from = self.reader_state.index_finish;
        self.reader_state.current_states = self.reader_state.start_states.clone();
        self.reader_state.last_accepted_states = StateSet::new(vec![]);

        if from >= self.reader_state.index_from {
            panic!();
        }

        res
    }

    fn build_token(&self) -> Token {
        let accepted_states = &self.reader_state.last_accepted_states;

        // println!("accepted_state_len: {}", accepted_states.len());
        // println!("accepted_states.is_empty: {}", accepted_states.is_empty());

        if accepted_states.is_empty() {
            panic!("chars: {:?}\nreader_state: {}", self.chars, self.reader_state);
        }

        let state = accepted_states.accepted_state().unwrap();
        let token_type = (*state).borrow().token_type.clone().unwrap();
        let mut literal = "".to_string();
        for idx in self.reader_state.index_from..self.reader_state.index_finish {
            literal.push(self.chars[idx]);
        }
        let token = Token::new(token_type, &literal, (*state).borrow().skip);

        // println!("token: {}", token);

        token
    }
}
