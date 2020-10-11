pub struct StateNext {
    need_cond: bool,
    cond: Rc<dyn Fn(char) -> bool>,
    next: State,
}