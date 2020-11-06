#[warn(non_upper_case_globals)]
pub struct NFA {
    start: Rc<RefCell<State>>,
    finish: Rc<RefCell<State>>,
}

impl NFA {
    pub fn new(start: Rc<RefCell<State>>, finish: Rc<RefCell<State>>) -> NFA {
        NFA { start, finish }
    }

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
}

impl ToString for NFA {
    fn to_string(&self) -> String {
        let mut state_set = StateSet::new(vec![]);
        let start = self.start.clone();
        let finish = self.finish.clone();

        let mut nexts_str = HashSet::<String>::new();

        self._to_string_inner(start.clone(), &mut state_set, &mut nexts_str);

        let mut s = String::new();

        write!(s, "[\n");
        write!(s, "start : {}\n", (*start).borrow().to_string());
        write!(s, "finish: {}\n", (*finish).borrow().to_string());
        write!(s, "states: {}\n", state_set.to_string());
        write!(s, "paths :\n");
        for nstr in nexts_str {
            write!(s, " {}\n", nstr);
        }
        write!(s, "]\n");

        s
    }
}

impl NFA {
    fn _to_string_inner(&self, from: Rc<RefCell<State>>, state_set: &mut StateSet, nexts_str: &mut HashSet<String>) {
        if state_set.add(from.clone()) {
            let state_str = (*from).borrow().to_string();

            let mut set2 = StateSet::new(vec![]);

            for next in &(*from).borrow().next_vec {
                nexts_str.insert(state_str.to_string() + &next.to_string());

                set2.add(next.next.clone());
                // self._to_string_inner(next.next.clone(), state_set, nexts_str);
            }

            for state in set2.states {
                self._to_string_inner(state, state_set, nexts_str);
            }
        }
    }

    pub fn chi_nfa() -> Rc<NFA> {
        let _int = NFA::new_by_string("int", Some(INT), false);
        let _float = NFA::new_by_string("float", Some(FLOAT), false);

        // [ \t\r\n\u000C]+  -> skip ;
        let _ws_c = NFA::new_by_string(" \t\r\n\u{000C}", None, false);
        let mut _ws = NFA::kleen_closure_plus(_ws_c);
        (*_ws.finish).borrow_mut().skip = true;

        let _bool = NFA::new_by_string("bool", Some(BOOL), false);
        let _boolean = NFA::new_by_string("boolean", Some(BOOL), false);

        let _pub = NFA::new_by_string("pub", Some(PUBLIC), false);
        let _public = NFA::new_by_string("public", Some(PUBLIC), false);

        let _pvt = NFA::new_by_string("pvt", Some(PRIVATE), false);
        let _private = NFA::new_by_string("private", Some(PRIVATE), false);

        let _prtc = NFA::new_by_string("prtc", Some(PROTECTED), false);
        let _protected = NFA::new_by_string("protected", Some(PROTECTED), false);

        let _fun = NFA::new_by_string("fun", Some(FUNCTION), false);
        let _function = NFA::new_by_string("function", Some(FUNCTION), false);

        let _let = NFA::new_by_string("let", Some(LET), false);
        let _mut = NFA::new_by_string("mut", Some(MUTABLE), false);
        let _char = NFA::new_by_string("char", Some(CHARACTER), false);
        let _override = NFA::new_by_string("override", Some(OVERRIDE), false);
        let _tailrec = NFA::new_by_string("tailrec", Some(TAILREC), false);
        let _class = NFA::new_by_string("class", Some(CLASS), false);

        let nfa_vec = vec![
            _int,
            _float,
            _ws,
            _bool, _boolean,
            _pub, _public,
            _pvt, _private,
            _prtc, _protected,
            _fun, _function,
            _let,
            _mut,
            _char,
            _override,
            _tailrec,
            _class,
        ];
        Rc::new(NFA::alternate(nfa_vec))
    }
}