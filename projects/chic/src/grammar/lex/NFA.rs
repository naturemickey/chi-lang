#[warn(non_upper_case_globals)]
pub struct NFA {
    start: Rc<RefCell<State>>,
    finish: Rc<RefCell<State>>,
}

impl NFA {
    pub fn new(start: Rc<RefCell<State>>, finish: Rc<RefCell<State>>) -> NFA {
        NFA { start, finish }
    }

    pub fn new_by_string(s: &str, token_type: Option<TokenType>) -> NFA {
        let chars = s.chars();
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

    pub fn alternate(mut nfa_vec: Vec<NFA>) -> NFA {
        let finish = State::new_finish();

        let mut next_vec = Vec::new();
        for mut nfa in nfa_vec {
            (*nfa.finish).borrow_mut().add_next(StateNext::new_no_cond(finish.clone()));
            next_vec.push(StateNext::new_no_cond(nfa.start));
        }
        let start = State::new_normal(next_vec);
        NFA::new(start, finish)
    }

    pub fn kleen_closure(mut nfa: NFA) -> NFA {
        let finish = State::new_finish();

        (*nfa.finish).borrow_mut().add_next(StateNext::new_no_cond(finish.clone()));
        (*nfa.finish).borrow_mut().add_next(StateNext::new_no_cond(nfa.start.clone()));

        let start = State::new_normal(vec![StateNext::new_no_cond(nfa.start), StateNext::new_no_cond(finish.clone())]);

        NFA::new(start, finish)
    }

    pub fn chi_nfa() -> Rc<NFA> {
        let _INT = NFA::new_by_string("int", Some(INT));
        let _FLOAT = NFA::new_by_string("float", Some(FLOAT));

        let nfa_vec = vec![_INT, _FLOAT, ];
        Rc::new(NFA::alternate(nfa_vec))
    }
}
