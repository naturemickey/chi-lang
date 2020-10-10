pub struct State {
    // 1. start; 2. finish; 0. normal
    _type: StateType,
    next_vec: Vec<StateNext>,
    token_type: Option<TokenType>,
}

impl State {
    pub fn new(_type: StateType, next_vec: Vec<StateNext>, token_type: Option<TokenType>) -> State {
        State { _type, next_vec, token_type }
    }
}
