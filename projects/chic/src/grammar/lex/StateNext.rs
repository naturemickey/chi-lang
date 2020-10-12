pub struct StateNext {
    need_cond: bool,
    cond: Rc<dyn Fn(char) -> bool>,
    next: Rc<State>,
}

impl StateNext {
    pub fn new_by_cond(cond: Rc<dyn Fn(char) -> bool>, next: Rc<State>) -> StateNext {
        StateNext { need_cond: false, cond, next }
    }
}