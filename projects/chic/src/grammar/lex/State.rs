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

    pub fn eq_state_vec(&self) -> Vec<Rc<RefCell<State>>> {
        let mut res = Vec::new();
        for next in &self.next_vec {
            if next.need_cond == false {
                res.push(next.next.clone());
            }
        }
        res
    }

    /*
    跳转到下一个状态。
    因为是NFA的状态，所以下一状态的个数是不确定的。
    */
    fn jump(&self, c: char) -> Vec<Rc<RefCell<State>>> {
        let mut v = Vec::new();
        for sn in &self.next_vec {
            match sn.jump(c) {
                Some(next) => {
                    v.push(sn.next.clone());

                    for state in &(*sn.next).borrow().eq_state_vec() {
                        v.push(state.clone());
                    }
                }
                None => {}
            }
        }
        v
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