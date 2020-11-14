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
            StateNextCond::NONE => None, // 在真正跳转的时候不应该走这里；空条件应该是等价状态处理的。
            StateNextCond::CHAR(_c) => {
                // println!("c & c : {} {}", c, _c);
                if c == *_c {
                    Some(self.next.clone())
                } else {
                    None
                }
            }
            StateNextCond::FN(f) => if (**f).borrow()(c) {
                Some(self.next.clone())
            } else {
                None
            },
        }
    }
}

enum StateNextCond {
    NONE,
    CHAR(char),
    FN(Rc<dyn Fn(char) -> bool>),
}

impl Display for StateNext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let next_str = (*self.next).borrow().to_string();

        match &self.cond {
            StateNextCond::NONE => write!(f, " -> {}", next_str),
            StateNextCond::CHAR(c) => write!(f, " -> {} : ({})", next_str, visible_char(c)),
            StateNextCond::FN(_) => write!(f, " -> {} : Fn", next_str), // 无法打印Fn的内容，有点糟糕。
        }
    }
}

fn visible_char(c: &char) -> String {
    let ch = c.clone() as u64;

    if ch >= 33 && ch <= 126 {
        c.to_string()
    } else if ch == '\t' as u64 {
        "\\t".to_string()
    } else if ch == '\r' as u64 {
        "\\r".to_string()
    } else if ch == '\n' as u64 {
        "\\n".to_string()
    } else {
        format!("\\u{:02X}",ch)
    }
}