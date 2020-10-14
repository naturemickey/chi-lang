pub struct StateSet {
    states: Vec<Rc<State>>,
}

impl StateSet {
    pub fn add(&mut self, state: Rc<State>) -> bool {
        for rc_state in &self.states {
            if Rc::as_ptr(rc_state) == Rc::as_ptr(&state) {
                return false;
            }
        }
        self.states.push(state);
        true
    }
}