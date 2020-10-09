pub struct State {
    // 1. start; 2. finish; 0. normal
    _type: StateType,
    nextVec: Vec<StateNext>,
    tokenType: Option<TokenType>,
}

impl State {
    pub fn new(_type: StateType, nextVec: Vec<StateNext>, tokenType: Option<TokenType>) -> State {
        State { _type, nextVec, tokenType }
    }
}
