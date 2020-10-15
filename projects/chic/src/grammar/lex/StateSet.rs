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
        }
        self.states.push(state);
        true
    }

    pub fn jump(&self, c: char) -> StateSet {
        let mut state_set = StateSet { states: Vec::new() };
        for state in &self.states {
            let st = &**state;
            let ss = st.borrow().jump(c);
            for s in ss {
                state_set.add(s.clone());
            }
        }
        state_set
    }

    pub fn acepted_states(&self) -> StateSet {
        let mut res = StateSet {states:Vec::new()};
        for state in &self.states {
            let st = &**state;
            let token_type = &(*st).borrow().token_type;
            if token_type.is_some() {
                res.states.push(state.clone());
            }
        }
        res
    }

    pub fn token_types(&self) -> Vec<TokenType> {
        let mut vec = Vec::new();

        for state in &self.states {
            let token_type = &(**state).borrow().token_type;
            match token_type {
                Some(tt) => vec.push(tt.clone()),
                None => {},
            }
        }

        vec
    }
}