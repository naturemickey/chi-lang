#[warn(non_upper_case_globals)]
pub struct NFA {
    start: Rc<State>,
    finish: Rc<State>,
}

impl NFA {
    pub fn new(start: Rc<State>, finish: Rc<State>) -> NFA {
        NFA { start, finish }
    }

    pub fn create_ty_chars(chars: Chars, token_type:TokenType) -> NFA {
        todo!()
    }

    pub fn alternate(nfa_vec: Vec<NFA>) -> NFA {
        todo!()
    }
}