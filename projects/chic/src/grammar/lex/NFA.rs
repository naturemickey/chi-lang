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

    pub fn new_by_fn(_fn: Rc<dyn Fn(char) -> bool>, token_type: Option<TokenType>, skip: bool) -> NFA {
        let finish = State::new_accepted(token_type, skip);
        let next = StateNext::new_by_fn(_fn, finish.clone());
        let start = State::new_normal(vec![next]);
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

    pub fn non_or_one(nfa: NFA) -> NFA {
        let next = StateNext::new_no_cond(nfa.finish.clone());
        nfa.start.borrow_mut().next_vec.push(next);
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
}

impl NFA {
    pub fn chi_nfa() -> Rc<NFA> {
        // Keywords
        let _int = NFA::new_by_string("int", Some(INT), false);
        let _float = NFA::new_by_string("float", Some(FLOAT), false);

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

        let mut keyworlds_nfas = vec![
            _int,
            _float,
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

        let mut separators_nfas = vec![
            NFA::new_by_string("(", Some(LPAREN), false),
            NFA::new_by_string(")", Some(RPAREN), false),
            NFA::new_by_string("{", Some(LBRACE), false),
            NFA::new_by_string("}", Some(RBRACE), false),
            NFA::new_by_string("[", Some(LBRACK), false),
            NFA::new_by_string("]", Some(RBRACK), false),
            NFA::new_by_string(";", Some(SEMI), false),
            NFA::new_by_string(",", Some(COMMA), false),
            NFA::new_by_string(".", Some(DOT), false),
        ];

        let mut operators_nfas = vec![
            NFA::new_by_string("=", Some(ASSIGN), false),
            NFA::new_by_string(">", Some(GT), false),
            NFA::new_by_string("<", Some(LT), false),
            NFA::new_by_string("!", Some(BANG), false),
            NFA::new_by_string("~", Some(TILDE), false),
            NFA::new_by_string("?", Some(QUESTION), false),
            NFA::new_by_string(":", Some(COLON), false),
            NFA::new_by_string("==", Some(EQUAL), false),
            NFA::new_by_string("<=", Some(LE), false),
            NFA::new_by_string(">=", Some(GE), false),
            NFA::new_by_string("!=", Some(NOTEQUAL), false),
            NFA::new_by_string("&&", Some(AND), false),
            NFA::new_by_string("||", Some(OR), false),
            NFA::new_by_string("++", Some(INC), false),
            NFA::new_by_string("--", Some(DEC), false),
            NFA::new_by_string("+", Some(ADD), false),
            NFA::new_by_string("-", Some(SUB), false),
            NFA::new_by_string("*", Some(MUL), false),
            NFA::new_by_string("/", Some(DIV), false),
            NFA::new_by_string("&", Some(BITAND), false),
            NFA::new_by_string("|", Some(BITOR), false),
            NFA::new_by_string("^", Some(CARET), false),
            NFA::new_by_string("%", Some(MOD), false),
            NFA::new_by_string("->", Some(ARROW), false),
            NFA::new_by_string("::", Some(COLONCOLON), false),
            NFA::new_by_string("+=", Some(ADD_ASSIGN), false),
            NFA::new_by_string("-=", Some(SUB_ASSIGN), false),
            NFA::new_by_string("*=", Some(MUL_ASSIGN), false),
            NFA::new_by_string("/=", Some(DIV_ASSIGN), false),
            NFA::new_by_string("&=", Some(AND_ASSIGN), false),
            NFA::new_by_string("|=", Some(OR_ASSIGN), false),
            NFA::new_by_string("^=", Some(XOR_ASSIGN), false),
            NFA::new_by_string("%=", Some(MOD_ASSIGN), false),
            NFA::new_by_string("<<=", Some(LSHIFT_ASSIGN), false),
            NFA::new_by_string(">>=", Some(RSHIFT_ASSIGN), false),
            NFA::new_by_string(">>>=", Some(URSHIFT_ASSIGN), false),
        ];

        let mut booleans_nfas = vec![
            NFA::new_by_string("true", Some(BooleanLiteral), false),
            NFA::new_by_string("false", Some(BooleanLiteral), false),
        ];

        let mut nfa_vec = Vec::new();
        nfa_vec.append(&mut keyworlds_nfas);
        nfa_vec.append(&mut separators_nfas);
        nfa_vec.append(&mut operators_nfas);
        nfa_vec.append(&mut booleans_nfas);

        nfa_vec.push(Self::_ws());
        nfa_vec.push(Self::_comment());
        nfa_vec.push(Self::_line_comment());

        Rc::new(NFA::alternate(nfa_vec))
    }

    fn _ws() -> NFA {
        // [ \t\r\n\u000C]+  -> skip ;
        let _ws_c = NFA::new_by_string(" \t\r\n\u{000C}", None, false);
        let mut _ws = NFA::kleen_closure_plus(_ws_c);
        (*_ws.finish).borrow_mut().skip = true;
        _ws
    }

    fn _comment() -> NFA {
        // '/*' .* '*/'      -> skip ;
        let start = NFA::new_by_string("/*", None, false);
        let finish = NFA::new_by_string("*/", Some(COMMENT), true);

        let middle = NFA::new_by_fn(Rc::new(|_| true), None, false);

        NFA::concatenate(vec![start, middle, finish])
    }

    fn _line_comment() -> NFA {
        // '//' ~[\r\n]*     -> skip ;
        let start = NFA::new_by_string("//", None, false);
        let rest = NFA::new_by_fn(Rc::new(|c| c != '\r' && c != '\n'), None, false);

        rest.finish.borrow_mut().skip = true;
        rest.finish.borrow_mut().token_type = Some(LINE_COMMENT);

        NFA::concatenate(vec![start, rest])
    }

    fn _integer_literal() -> NFA {
        let nfa = Self::_decimal_integer_literal();

        // 未来扩展，目前先做一个十进制

        nfa
    }
}

impl NFA {
    // fragment
    fn _single_character() -> NFA {
        // ~['\\\r\n]
        Self::new_by_fn(Rc::new(|c| c != '\'' && c != '\\' && c != '\r' && c != '\n'), None, false)
    }

    fn _decimal_integer_literal() -> NFA {
        // fragment
        // DecimalIntegerLiteral
        //     : DecimalNumeral [lL]?
        //     ;
        let decimal_numeral = Self::_decimal_numeral();
        let l_L_ = Self::non_or_one(Self::new_by_fn(Rc::new(|c| c == 'l' || c == 'L'), None, false));

        Self::concatenate(vec![decimal_numeral, l_L_]);
    }

    fn _decimal_numeral() -> NFA {
        // DecimalNumeral
        //     : '0'
        //     | NonZeroDigit Digits?
        //     ;
        let zero = Self::new_by_string("0", None, false);
        let non_zero = Self::_non_zero_digit();
        let digits = Self::non_or_one(Self::_digits());

        Self::alternate(vec![zero, Self::concatenate(vec![non_zero, digits])])
    }

    fn _octal_digit() -> NFA {
        // [0-7]
        Self::new_by_fn(Rc::new(|c| '0' <= c && c <= '7'), None, false)
    }

    fn _hex_digit() -> NFA {
        // [0-9a-fA-F]
        Self::new_by_fn(Rc::new(|c| ('0' <= c && c <= '9') ||
            ('a' <= c && c <= 'f') ||
            ('A' <= c && c <= 'F')
        ), None, false)
    }

    fn _non_zero_digit() -> NFA {
        // [1-9]
        Self::new_by_fn(Rc::new(|c| '1' <= c && c <= '9'), None, false)
    }

    fn _digits() -> NFA {
        // [0-9]+
        let nfa = Self::new_by_fn(Rc::new(|c| '0' <= c && c <= '9'), None, false);
        Self::kleen_closure_plus(nfa)
    }
}