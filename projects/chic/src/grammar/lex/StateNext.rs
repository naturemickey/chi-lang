pub struct StateNext {
    cond: StateNextCond,
    next: Rc<RefCell<State>>,
}

impl StateNext {
    pub fn new_no_cond(next: Rc<RefCell<State>>) -> StateNext {
        Self { cond: StateNextCond::NONE, next }
    }

    pub fn new_by_char(c: char, next: Rc<RefCell<State>>) -> StateNext {
        Self { cond: StateNextCond::CHAR(c), next }
    }

    pub fn new_by_fn(_fn: Rc<dyn Fn(char) -> bool>, next: Rc<RefCell<State>>) -> StateNext {
        Self { cond: StateNextCond::FN(_fn), next }
    }

    pub fn jump(&self, c: char) -> Option<Rc<RefCell<State>>> {
        match &self.cond {
            StateNextCond::NONE => None,
            StateNextCond::CHAR(_c) => if c == *_c {
                Some(self.next.clone())
            } else {
                None
            },
            StateNextCond::FN(f) => if (**f).borrow()(c) {
                Some(self.next.clone())
            } else {
                None
            },
        }

        //todo!()
        // if self.need_cond && (*self.cond)(c) {
        //     Some(self.next.clone())
        // } else {
        //     None
        // }
    }
}

enum StateNextCond {
    NONE,
    CHAR(char),
    FN(Rc<dyn Fn(char) -> bool>),
}