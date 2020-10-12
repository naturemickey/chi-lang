#[warn(non_upper_case_globals)]
pub struct NFA {
    start: Rc<State>,
    finish: Rc<State>,
}

impl NFA {
    pub fn new(start: Rc<State>, finish: Rc<State>) -> NFA {
        NFA { start, finish }
    }

    pub fn create_ty_chars(chars: Chars, token_type: Option<TokenType>) -> NFA {
        let finish = State::new_accepted(token_type);
        let mut start = State::new_normal(vec![]);

        let mut is_first = true;
        for c in chars.rev() {
            let next = if is_first {
                is_first = false;
                StateNext::new_by_cond(Rc::new(move |_c| _c == c), finish.clone())
            } else {
                StateNext::new_by_cond(Rc::new(move |_c| _c == c), start.clone())
            };
            start = State::new_normal(vec![next]);
        }

        NFA::new(start, finish)
    }

    pub fn alternate(nfa_vec: Vec<NFA>) -> NFA {
        todo!()
    }

    pub fn kleen_closure(nfa: NFA) -> NFA {
        todo!()
    }
}