pub struct State {
    next_vec: Vec<StateNext>,
    token_type: Option<TokenType>, // 终止状态时为Some，否则为None
}

impl State {
    fn new(next_vec: Vec<StateNext>, token_type: Option<TokenType>) -> State {
        State { next_vec, token_type }
    }

    pub fn new_normal(next_vec: Vec<StateNext>) -> State {
        Self::new(next_vec, None)
    }

    pub fn new_accepted(token_type: Option<TokenType>) -> State {
        Self::new(Vec::new(), token_type)
    }

    pub fn is_accepted(&self) -> bool {
        match self.token_type {
            Some(_) => true,
            None => false
        }
    }
}
