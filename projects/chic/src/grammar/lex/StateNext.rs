
pub struct StateNext {
    need_cond: bool,
    cond: Rc<Fn(char) -> bool>,
    next: State,
}