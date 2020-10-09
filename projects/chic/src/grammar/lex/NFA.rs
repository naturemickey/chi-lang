#[warn(non_upper_case_globals)]
pub struct NFA {
    start: Box<State>,
    finish: Box<State>,
}

impl NFA {
    pub fn new(start: Box<State>, finish: Box<State>) -> NFA {
        NFA { start, finish }
    }

    pub fn create_ty_chars(chars: Chars) -> NFA {
        panic!()
    }

    pub fn alternate(nfaVec: Vec<NFA>) -> NFA {
        panic!()
    }
}