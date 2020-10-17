#[warn(non_upper_case_globals)]
pub struct NFA {
    start: Rc<RefCell<State>>,
    finish: Rc<RefCell<State>>,
}

impl NFA {
    pub fn new(start: Rc<RefCell<State>>, finish: Rc<RefCell<State>>) -> NFA {
        NFA { start, finish }
    }

    // pub fn new_impossible() -> NFA {
    //     // #![feature(new_uninit)]
    //     // NFA { start: Rc::new_zeroed(), finish: Rc::new_zeroed() }
    //     let temp_state = State::new_finish();
    //     NFA { start: temp_state.clone(), finish: temp_state.clone() }
    // }

    pub fn new_by_string(s: &str, token_type: Option<TokenType>, skip: bool) -> NFA {
        let chars = s.chars();
        let finish = State::new_accepted(token_type, skip);
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

    pub fn concatenate(mut nfa_vec: Vec<NFA>) -> NFA {
        if nfa_vec.len() <= 0 {
            panic!("impossible.");
        }

        if nfa_vec.len() == 1 {
            // 这里无法return nfa_vec[0];也是很蛋疼的。现在这种方式类似于C++11的移动构造。
            return NFA::new(nfa_vec[0].start.clone(), nfa_vec[0].finish.clone());
        }

        // 此处初始化没有意义，只是为了应付编译器
        let mut start = State::new_finish();
        let mut finish = State::new_finish();
        let mut first = true;
        for mut nfa in nfa_vec {
            if first {
                first = false;
                start = nfa.start.clone();
            } else {
                (*finish).borrow_mut().add_next(StateNext::new_no_cond(nfa.start.clone()));
                finish = nfa.finish.clone();
            }
        }
        NFA { start, finish }
    }

    pub fn kleen_closure(nfa: NFA) -> NFA {
        let finish = State::new_finish();

        (*nfa.finish).borrow_mut().add_next(StateNext::new_no_cond(finish.clone()));
        (*nfa.finish).borrow_mut().add_next(StateNext::new_no_cond(nfa.start.clone()));

        let start = State::new_normal(vec![StateNext::new_no_cond(nfa.start), StateNext::new_no_cond(finish.clone())]);

        NFA::new(start, finish)
    }

    pub fn chi_nfa() -> Rc<NFA> {
        let _INT = NFA::new_by_string("int", Some(INT), false);
        let _FLOAT = NFA::new_by_string("float", Some(FLOAT), false);


        let nfa_vec = vec![_INT, _FLOAT, ];
        Rc::new(NFA::alternate(nfa_vec))
    }
}
