
pub struct StateNext {
    need_cond: bool,
    cond: Box<Fn(char) -> bool>,
    next: State,
}