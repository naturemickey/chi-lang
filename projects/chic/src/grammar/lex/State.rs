// #[derive(Hash, PartialEq, Eq)]
pub struct State {
    next_vec: Vec<StateNext>,
    token_type: Option<TokenType>, // 终止状态时为Some，否则为None
}

impl State {
    fn new(next_vec: Vec<StateNext>, token_type: Option<TokenType>) -> Rc<State> {
        Rc::new(State { next_vec, token_type })
    }

    pub fn new_normal(next_vec: Vec<StateNext>) -> Rc<State> {
        Self::new(next_vec, None)
    }

    pub fn new_accepted(token_type: Option<TokenType>) -> Rc<State> {
        Self::new(Vec::new(), token_type)
    }

    pub fn is_accepted(&self) -> bool {
        match self.token_type {
            Some(_) => true,
            None => false
        }
    }
}

// impl Hash for State {
//     fn hash<H: std::hash::Hasher>(&self, _: &mut H) {
//         todo!()
//     }
// }

// impl PartialEq<State> for State {
//     fn eq(&self, that: &State) -> bool {
//         &&self == &&that
//     }
// }