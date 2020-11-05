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

    // pub fn add_all(&mut self, states: &Vec<Rc<RefCell<State>>>) {
    //     for state in states {
    //         self.add(state.clone());
    //     }
    // }

    pub fn merge(&mut self, other: StateSet) {
        for state in other.states {
            self.add(state.clone());
        }
    }

    pub fn jump(&self, c: char) -> StateSet {
        let mut state_set = StateSet { states: Vec::new() };
        for state in &self.states {
            let st = &**state;
            let ss = st.borrow().jump(c);

            state_set.merge(ss);
        }
        state_set
    }

    pub fn accepted_states(&self) -> StateSet {
        let mut res = StateSet { states: Vec::new() };
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
                None => {}
            }
        }

        vec
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }
}

impl ToString for StateSet {
    fn to_string(&self) -> String {
        let mut s = String::new();

        write!(s, "[");
        for state in &self.states {
            write!(s, "{}", (**state).borrow().to_string());
            write!(s, ", ");
        }
        write!(s, "]");

        s
    }
}