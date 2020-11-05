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
        if s.len() == 0 {
            panic!("不允许出现空的词法。");
        }
        let chars = s.chars();
        let finish = State::new_accepted(token_type, skip);
        let mut start = State::new_normal(vec![]); // 此处初始化的start值是一个浪费。

        let mut is_first = true;
        for c in chars.rev() { // 反着遍历
            let next = if is_first { // 第一个，即是最后一个
                is_first = false;
                StateNext::new_by_char(c, finish.clone())
            } else {
                StateNext::new_by_char(c, start.clone())
            };
            start = State::new_normal(vec![next]);
        }

        NFA::new(start, finish)
    }

    pub fn alternate(nfa_vec: Vec<NFA>) -> NFA {
        let finish = State::new_finish();

        let mut next_vec = Vec::new();
        for nfa in nfa_vec {
            (*nfa.finish).borrow_mut().add_next(StateNext::new_no_cond(finish.clone()));
            next_vec.push(StateNext::new_no_cond(nfa.start));
        }
        let start = State::new_normal(next_vec);
        NFA::new(start, finish)
    }

    pub fn concatenate(nfa_vec: Vec<NFA>) -> NFA {
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
        for nfa in nfa_vec {
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

    pub fn kleen_closure_plus(nfa: NFA) -> NFA {
        (*nfa.finish).borrow_mut().add_next(StateNext::new_no_cond(nfa.start.clone()));
        nfa
    }

    pub fn chi_nfa() -> Rc<NFA> {
        let _int = NFA::new_by_string("int", Some(INT), false);
        let _float = NFA::new_by_string("float", Some(FLOAT), false);

        // [ \t\r\n\u000C]+  -> skip ;
        let _ws_c = NFA::new_by_string(" \t\r\n\u{000C}", None, false);
        let mut _ws = NFA::kleen_closure_plus(_ws_c);
        (*_ws.finish).borrow_mut().skip = true;


        let nfa_vec = vec![_int, _float, _ws];
        Rc::new(NFA::alternate(nfa_vec))
    }

    // 为了调试，打印出来看看
    // pub fn print(&self) {
    //     let state_str_vec = Self::break_state(self.start.clone());
    //
    //     for state_str in state_str_vec {
    //         println!["{}", &state_str];
    //     }
    // }
    //
    // pub fn break_state(state:Rc<RefCell<State>>) -> Vec<String> {
    //     (*state).borrow().to_string_vec()
    // }
}

impl ToString for NFA {
    fn to_string(&self) -> String {
        let mut state_set = StateSet::new(vec![]);
        let start = self.start.clone();
        let finish = self.finish.clone();

        let mut nexts_str = HashSet::<String>::new();

        self._to_string_inner(start.clone(), &mut tate_set, &mut nexts_str);

        let mut s = String::new();

        write!(&s, "[\n");
        write!(&s, "start : {}\n", start.borrow().borrow().to_string());
        write!(&s, "finish: {}\n", finish.borrow().borrow().to_string());
        write!(&s, "states: {}\n", state_set.to_string());
        write!(&s, "paths :\n");
        for nstr in nexts_str {
            write!(&s, " {}\n", nstr);
        }

        s
    }
}

impl NFA {
    fn _to_string_inner(&self, from: Rc<RefCell<State>>, state_set: &mut StateSet, nexts_str: &mut HashSet<String>) {
        unimplemented!()
    }
}