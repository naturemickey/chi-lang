// #[derive(Hash, PartialEq, Eq)]
pub struct State {
    next_vec: Vec<StateNext>,
    token_type: Option<TokenType>, // 终止状态时为Some，否则为None
}

impl State {
    fn new(next_vec: Vec<StateNext>, token_type: Option<TokenType>) -> Rc<RefCell<State>> {
        Rc::new(RefCell::new(State { next_vec, token_type }))
    }

    pub fn new_normal(next_vec: Vec<StateNext>) -> Rc<RefCell<State>> {
        Self::new(next_vec, None)
    }

    pub fn new_accepted(token_type: Option<TokenType>) -> Rc<RefCell<State>> {
        Self::new(Vec::new(), token_type)
    }

    pub fn new_finish() -> Rc<RefCell<State>> {
        Self::new(Vec::new(), None)
    }

    pub fn is_accepted(&self) -> bool {
        match self.token_type {
            Some(_) => true,
            None => false
        }
    }

    pub fn add_next(&mut self, next: StateNext) {
        self.next_vec.push(next);
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