pub struct StateSet {
    states: Vec<Rc<RefCell<State>>>,
}

impl StateSet {
    pub fn new(states: Vec<Rc<RefCell<State>>>) -> StateSet {
        Self { states }
    }

    pub fn add(&mut self, state: Rc<RefCell<State>>) -> bool {
        for rc_state in &self.states {
            if Rc::as_ptr(rc_state) == Rc::as_ptr(&state) {
                return false;
            }
            if (**rc_state).borrow().id == (*state).borrow().id {
                return false;
            }
        }
        self.states.push(state);
        true
    }

    pub fn merge(&mut self, other: StateSet) {
        for state in other.states {
            self.add(state.clone());
        }
    }

    pub fn jump(&self, c: char) -> StateSet {
        // println!("current state_set length is {}", self.states.len());
        let mut state_set = StateSet { states: Vec::new() };
        for state in &self.states {
            let ss = (**state).borrow().jump(c);

            // println!("ss length is {}", ss.states.len());
            // println!("ss: {}", ss);

            state_set.merge(ss);
        }
        state_set
    }

    pub fn accepted_states(&self) -> StateSet {
        let mut res = StateSet::new(vec![]);
        for state in &self.states {
            let token_type = &(**state).borrow().token_type;
            if token_type.is_some() {
                res.add(state.clone());
            }
        }
        res
    }

    /// 在当前集合中找到级别最高的一个返回(数字越小的级别越高)。
    pub fn accepted_state(&self) -> Option<Rc<RefCell<State>>> {
        let mut result = None;
        for state in &self.states {
            match &(**state).borrow().token_type {
                Some(token_type) => {
                    match &result {
                        None => {
                            result = Some(state.clone());
                        }
                        Some(rtt) => {
                            let l1 = match &(*rtt.clone()).borrow().token_type {
                                None=> i32::MAX,
                                Some(tt) => tt.get_level()
                            };

                            let l2 = token_type.get_level();

                            if l1 > l2 {
                                result = Some(state.clone());
                            }
                        }
                    }
                }
                None => {}
            }
        }
        result
    }

    pub fn token_types(&self) -> Vec<TokenType> {
        let mut vec = Vec::new();

        for state in &self.states {
            let token_type = &(**state).borrow().token_type;
            match token_type {
                Some(tt) => vec.push(tt.clone()),
                None => {}
            }
        }

        vec
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }
}

impl Display for StateSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for state in &self.states {
            write!(f, "{}", (**state).borrow())?;
            write!(f, ", ")?;
        }
        write!(f, "]")
    }
}

impl Clone for StateSet {
    fn clone(&self) -> Self {
        // println!("self: {}", self);
        let res = Self { states: self.states.clone() };

        // println!("self.clone: {}", res);
        res
    }
}