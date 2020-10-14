pub struct StateNext {
    need_cond: bool,
    cond: Rc<dyn Fn(char) -> bool>,
    next: Rc<RefCell<State>>,
}

impl StateNext {
    pub fn new_by_cond(cond: Rc<dyn Fn(char) -> bool>, next: Rc<RefCell<State>>) -> StateNext {
        Self { need_cond: true, cond, next }
    }

    pub fn new_no_cond(next: Rc<RefCell<State>>) -> StateNext {
        Self { need_cond: false, cond: Rc::new(|_| true), next }
    }
}