pub struct StateSet {
    states: Vec<Rc<RefCell<State>>>,
}

impl StateSet {
    pub fn new(states: Vec<Rc<RefCell<State>>>) -> StateSet {
        Self { states }
    }

    // pub fn add(&mut self, state: Rc<State>) -> bool {
    //     for rc_state in &self.states {
    //         if Rc::as_ptr(rc_state) == Rc::as_ptr(&state) {
    //             return false;
    //         }
    //     }
    //     self.states.push(state);
    //     true
    // }

    pub fn jump(&self, c: char) -> StateSet {
        let mut states: Vec<Rc<RefCell<State>>> = Vec::new();
        for state in &self.states {
            let st = &**state;
            let ss = st.borrow().jump(c);
            for s in ss {
                states.push(s);
            }
        }
        todo!()
    }
}