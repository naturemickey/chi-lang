// #[derive(Hash, PartialEq, Eq)]
pub struct State {
    next_vec: Vec<StateNext>,
    /// 终止状态时为Some，否则为None
    token_type: Option<TokenType>,
    skip: bool,
}

impl State {
    fn new(next_vec: Vec<StateNext>, token_type: Option<TokenType>, skip: bool) -> Rc<RefCell<State>> {
        Rc::new(RefCell::new(State { next_vec, token_type, skip }))
    }

    pub fn new_normal(next_vec: Vec<StateNext>) -> Rc<RefCell<State>> {
        Self::new(next_vec, None, false)
    }

    pub fn new_accepted(token_type: Option<TokenType>, skip: bool) -> Rc<RefCell<State>> {
        Self::new(Vec::new(), token_type, skip)
    }

    pub fn new_finish() -> Rc<RefCell<State>> {
        Self::new(Vec::new(), None, false)
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
                    v.push(next.clone());

                    for state in &(*next).borrow().eq_state_vec() {
                        v.push(state.clone());
                    }
                }
                None => {}
            }
        }
        v
    }
    // pub fn to_string_vec(&self) -> Vec<String> {
    //     let mut sv = Vec::new();
    //     let self_str =
    //         match &self.token_type {
    //             Some(tt) => "s(".to_string() + &tt.to_string() + ")",
    //             None => "ns".to_string()
    //         };
    //
    //     for next in &self.next_vec {
    //         let mut next_str = "".to_string();
    //
    //         if next.need_cond {
    //             next_str.push_str("()");
    //         }
    //
    //         next_str.push_str("-->");
    //
    //         let state = (*next.next).borrow();
    //
    //         let state_string_vec = state.to_string_vec();
    //
    //         for state_string in state_string_vec {
    //             let mut s = "".to_string();
    //             s.push_str(&self_str);
    //             s.push_str(&next_str);
    //             s.push_str(&state_string);
    //             sv.push(s);
    //         }
    //     }
    //     sv
    // }
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