static ATOMIC_USIZE: AtomicUsize = AtomicUsize::new(0);

// #[derive(Hash, PartialEq, Eq)]
pub struct State {
    id: usize,
    next_vec: Vec<StateNext>,
    /// 终止状态时为Some，否则为None
    token_type: Option<TokenType>,
    skip: bool,
}

impl State {
    fn new(next_vec: Vec<StateNext>, token_type: Option<TokenType>, skip: bool) -> Rc<RefCell<State>> {
        let id = ATOMIC_USIZE.fetch_add(1, Ordering::SeqCst);
        Rc::new(RefCell::new(State { id, next_vec, token_type, skip }))
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
        // 这个match纯属做个校验，不过没啥用。
        match next.cond {
            StateNextCond::NONE => {
                for n in &self.next_vec {
                    match n.cond {
                        StateNextCond::NONE => {
                            if Rc::ptr_eq(&next.next, &n.next) {
                                panic!()
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        self.next_vec.push(next);
    }

    /**
    等价状态集合——不需要条件就可以跳转的状态的集合。
    */
    pub fn eq_state_vec(&self) -> StateSet {
        let mut state_set = StateSet::new(vec![]);

        let mut stack = Vec::new();

        for next in &self.next_vec {
            match next.cond {
                StateNextCond::NONE => { stack.push(next.next.clone()) }
                _ => {}
            }
        }

        while stack.len() > 0 {
            match stack.pop() {
                Some(state) => {
                    if state_set.add(state.clone()) {
                        for next in &(*state).borrow().next_vec {
                            match next.cond {
                                StateNextCond::NONE => { stack.push(next.next.clone()) }
                                _ => {}
                            }
                        }
                    }
                },
                None => {
                    panic!("不可能为None");
                }
            }
        }

        // for next in &self.next_vec {
        //     match next.cond {
        //         StateNextCond::NONE => {
        //             if self.id != (*next.next).borrow().id {
        //                 if state_set.add(next.next.clone()) { // 如果能添加，就认为是新的，于是递归；
        //                     println!("xxxxxxxx");
        //                     state_set.merge((*next.next.clone()).borrow().eq_state_vec());
        //                     println!("yyyyyyyy");
        //                 }
        //             }
        //         },
        //         _ => {},
        //     }
        // }

        // println!("state_set: {}", state_set);

        state_set
    }

    /**
    跳转到下一个状态。
    因为是NFA的状态，所以下一状态的个数是不确定的。
    */
    fn jump(&self, c: char) -> StateSet {
        // println!("self states: {}", self);

        let mut state_set = StateSet::new(vec![]);

        for sn in &self.next_vec {
            // println!("sn: {}, c: {}", sn, c);
            match sn.jump(c) {
                Some(next) => {
                    if state_set.add(next.clone()) {
                        state_set.merge((*next).borrow().eq_state_vec());
                    }
                }
                None => {}
            }
        }

        // println!("state_set: {}", state_set);

        state_set
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}", &self.id)?;

        match &self.token_type {
            Some(tt) => { write!(f, ", {}", tt)?; }
            None => {}
        };

        if self.skip {
            write!(f, ", skip")?;
        }

        write!(f, ")")
    }
}