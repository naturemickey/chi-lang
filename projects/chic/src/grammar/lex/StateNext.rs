pub struct StateNext {
    need_cond: bool,
    cond: Rc<dyn Fn(char) -> bool>,
    next: Rc<RefCell<State>>,
}

impl StateNext {
    pub fn new_by_cond(cond: Rc<dyn Fn(char) -> bool>, next: Rc<RefCell<State>>) -> StateNext {
        StateNext { need_cond: false, cond, next }
    }

    pub fn new(next: Rc<RefCell<State>>) -> StateNext {
        Self::new_by_cond(Rc::new(|c| true), next)
    }
}