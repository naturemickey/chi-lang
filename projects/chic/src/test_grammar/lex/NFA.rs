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
                finish = nfa.finish.clone();
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
        (*nfa.start).borrow_mut().next_vec.push(next);
        nfa
    }
}

impl Display for NFA {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut state_set = StateSet::new(vec![]);
        let start = self.start.clone();
        let finish = self.finish.clone();

        let mut nexts_str = HashSet::<String>::new();

        self._to_string_inner(start.clone(), &mut state_set, &mut nexts_str);

        write!(f, "[\n")?;
        write!(f, "start : {}\n", (*start).borrow())?;
        write!(f, "finish: {}\n", (*finish).borrow())?;
        write!(f, "states: {}\n", state_set)?;
        write!(f, "paths :\n")?;
        for nstr in nexts_str {
            write!(f, " {}\n", nstr)?;
        }
        write!(f, "]\n")
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
        let mut keyworlds_nfas = vec![
            NFA::new_by_string("int", Some(INT), false),
            NFA::new_by_string("float", Some(FLOAT), false),
            NFA::new_by_string("bool", Some(BOOL), false),
            NFA::new_by_string("boolean", Some(BOOL), false),
            NFA::new_by_string("pub", Some(PUBLIC), false),
            NFA::new_by_string("public", Some(PUBLIC), false),
            NFA::new_by_string("pvt", Some(PRIVATE), false),
            NFA::new_by_string("private", Some(PRIVATE), false),
            NFA::new_by_string("prtc", Some(PROTECTED), false),
            NFA::new_by_string("protected", Some(PROTECTED), false),
            NFA::new_by_string("fun", Some(FUNCTION), false),
            NFA::new_by_string("function", Some(FUNCTION), false),
            NFA::new_by_string("let", Some(LET), false),
            NFA::new_by_string("mut", Some(MUTABLE), false),
            NFA::new_by_string("char", Some(CHARACTER), false),
            NFA::new_by_string("override", Some(OVERRIDE), false),
            NFA::new_by_string("tailrec", Some(TAILREC), false),
            NFA::new_by_string("class", Some(CLASS), false),
            NFA::new_by_string("object", Some(OBJECT), false),
            NFA::new_by_string("impl", Some(IMPLEMENTS), false),
            NFA::new_by_string("implements", Some(IMPLEMENTS), false),
            NFA::new_by_string("pkg", Some(PACKAGE), false),
            NFA::new_by_string("package", Some(PACKAGE), false),
            NFA::new_by_string("def", Some(DEFINE), false),
            NFA::new_by_string("define", Some(DEFINE), false),
            NFA::new_by_string("data", Some(DATA), false),
            NFA::new_by_string("catch", Some(CATCH), false),
            NFA::new_by_string("const", Some(CONST), false),
            NFA::new_by_string("continue", Some(CONTINUE), false),
            NFA::new_by_string("default", Some(DEFAULT), false),
            NFA::new_by_string("do", Some(DO), false),
            NFA::new_by_string("if", Some(IF), false),
            NFA::new_by_string("else", Some(ELSE), false),
            NFA::new_by_string("enum", Some(ENUM), false),
            NFA::new_by_string("extends", Some(EXTENDS), false),
            NFA::new_by_string("final", Some(FINAL), false),
            NFA::new_by_string("finally", Some(FINALLY), false),
            NFA::new_by_string("for", Some(FOR), false),
            NFA::new_by_string("goto", Some(GOTO), false),
            NFA::new_by_string("import", Some(IMPORT), false),
            NFA::new_by_string("instanceof", Some(INSTANCEOF), false),
            NFA::new_by_string("interface", Some(INTERFACE), false),
            NFA::new_by_string("long", Some(LONG), false),
            NFA::new_by_string("native", Some(NATIVE), false),
            NFA::new_by_string("new", Some(NEW), false),
            NFA::new_by_string("return", Some(RETURN), false),
            NFA::new_by_string("short", Some(SHORT), false),
            NFA::new_by_string("static", Some(STATIC), false),
            NFA::new_by_string("strictfp", Some(STRICTFP), false),
            NFA::new_by_string("super", Some(SUPER), false),
            NFA::new_by_string("switch", Some(SWITCH), false),
            NFA::new_by_string("synchronized", Some(SYNCHRONIZED), false),
            NFA::new_by_string("this", Some(THIS), false),
            NFA::new_by_string("throw", Some(THROW), false),
            NFA::new_by_string("throws", Some(THROWS), false),
            NFA::new_by_string("transient", Some(TRANSIENT), false),
            NFA::new_by_string("try", Some(TRY), false),
            NFA::new_by_string("void", Some(VOID), false),
            NFA::new_by_string("volatile", Some(VOLATILE), false),
            NFA::new_by_string("while", Some(WHILE), false),
            NFA::new_by_string("trait", Some(TRAIT), false),
            NFA::new_by_string("with", Some(WITH), false),
            NFA::new_by_string("use", Some(USE), false),
            NFA::new_by_string("sealed", Some(SEALED), false),
            NFA::new_by_string("self", Some(SELF), false),
            NFA::new_by_string("match", Some(MATCH), false),
            NFA::new_by_string("abstract", Some(ABSTRACT), false),
            NFA::new_by_string("assert", Some(ASSERT), false),
            NFA::new_by_string("break", Some(BREAK), false),
            NFA::new_by_string("byte", Some(BYTE), false),
            NFA::new_by_string("case", Some(CASE), false),
            NFA::new_by_string("val", Some(VAL), false),
            NFA::new_by_string("var", Some(VAR), false),
            NFA::new_by_string("type", Some(TYPE), false),
            NFA::new_by_string("lazy", Some(LAZY), false),
            NFA::new_by_string("implicit", Some(IMPLICIT), false),
            NFA::new_by_string("yield", Some(YIELD), false),
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

        nfa_vec.push(Self::_integer_literal());
        nfa_vec.push(Self::_character_literal());
        nfa_vec.push(Self::_identifier());

        nfa_vec.push(Self::_ws());
        nfa_vec.push(Self::_comment());
        nfa_vec.push(Self::_line_comment());

        for nfa in &nfa_vec {
            if nfa.finish.borrow_mut().token_type.is_none() {
                panic!("there is no finish state of the nfa: {}", nfa);
            }
        }

        Rc::new(NFA::alternate(nfa_vec))
    }

    pub fn _identifier() -> NFA {
        // Identifier               : IdentifierStart IdentifierPart*   ;
        // fragment IdentifierStart : [_A-Za-z]                         ;
        // fragment IdentifierPart  : [_A-Za-z0-9]                      ;

        let start = NFA::new_by_fn(Rc::new(|c| c == '_' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')), None, false);
        let parts = NFA::kleen_closure(
            NFA::new_by_fn(Rc::new(|c| c == '_' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || ('0' <= c && c <= '9')), None, false)
        );

        let id = NFA::concatenate(vec![start, parts]);
        (*id.finish).borrow_mut().token_type = Some(Identifier);

        id
    }

    fn _ws() -> NFA {
        // [ \t\r\n\u000C]+  -> skip ;
        // let _ws_c = NFA::new_by_string(" \t\r\n\u{000C}", None, false);
        let _ws_c = NFA::new_by_fn(Rc::new(|c| c == ' ' || c == '\t' || c == '\r' || c == '\n' || c == '\u{000C}'), None, false);
        let mut _ws = NFA::kleen_closure_plus(_ws_c);
        (*_ws.finish).borrow_mut().skip = true;
        (*_ws.finish).borrow_mut().token_type = Some(WS);
        _ws
    }

    fn _comment() -> NFA {
        // '/*' .* '*/'      -> skip ;
        let start = NFA::new_by_string("/*", None, false);
        let finish = NFA::new_by_string("*/", Some(COMMENT), true);

        let middle = NFA::kleen_closure(NFA::new_by_fn(Rc::new(|_| true), None, false));

        NFA::concatenate(vec![start, middle, finish])
    }

    fn _line_comment() -> NFA {
        // '//' ~[\r\n]*     -> skip ;
        let start = NFA::new_by_string("//", None, false);
        let rest = NFA::kleen_closure(NFA::new_by_fn(Rc::new(|c| c != '\r' && c != '\n'), None, false));

        (*rest.finish).borrow_mut().skip = true;
        (*rest.finish).borrow_mut().token_type = Some(LINE_COMMENT);

        NFA::concatenate(vec![start, rest])
    }

    fn _integer_literal() -> NFA {
        let nfa = Self::_decimal_integer_literal();

        // 未来扩展，目前先做一个十进制

        (*nfa.finish).borrow_mut().token_type = Some(IntegerLiteral);

        nfa
    }

    fn _character_literal() -> NFA {
        // CharacterLiteral
        // 	:	'\'' SingleCharacter '\''
        // 	|	'\'' EscapeSequence '\''
        // 	;
        let prefix_and_suffix = || Self::new_by_string("'", None, false);

        let nfa = Self::alternate(vec![
            Self::concatenate(vec![prefix_and_suffix(), Self::_single_character(), prefix_and_suffix()]),
            Self::concatenate(vec![prefix_and_suffix(), Self::_escape_sequence(), prefix_and_suffix()])
        ]);

        nfa.finish.borrow_mut().token_type = Some(CHARACTER);

        nfa
    }
}

impl NFA {
    // fragment

    fn _escape_sequence() -> NFA {
        // fragment
        // EscapeSequence
        // 	:	'\\' [btnfr"'\\]
        // 	|	OctalEscape
        //  |   UnicodeEscape
        // 	;
        Self::alternate(vec![
            Self::concatenate(vec![
                Self::new_by_string("\\", None, false),
                Self::new_by_fn(Rc::new(|c| "btnfr\"'\\".contains(c)), None, false),
            ]),
            Self::_octal_escape(),
            Self::_unicode_escape(),
        ])
    }

    fn _octal_escape() -> NFA {
        // fragment
        // OctalEscape
        // 	:	'\\{' OctalDigit '}'
        // 	|	'\\{' OctalDigit OctalDigit '}'
        // 	|	'\\{' ZeroToThree OctalDigit OctalDigit '}'
        // 	;
        let prefix = || Self::new_by_string("\\\\{", None, false);
        let suffix = || Self::new_by_string("}", None, false);

        Self::concatenate(vec![
            Self::concatenate(vec![prefix(), Self::_octal_digit(), suffix()]),
            Self::concatenate(vec![prefix(), Self::_octal_digit(), Self::_octal_digit(), suffix()]),
            Self::concatenate(vec![prefix(), Self::_zero_to_three(), Self::_octal_digit(), Self::_octal_digit(), suffix()]),
        ])
    }

    fn _zero_to_three() -> NFA {
        Self::new_by_string("0123", None, false)
    }

    fn _unicode_escape() -> NFA {
        // fragment
        // UnicodeEscape
        //     :   '\\' 'u'+ '{' HexDigit HexDigit HexDigit HexDigit '}'
        //     ;
        let prefix = Self::concatenate(vec![
            Self::new_by_string("\\\\", None, false),
            Self::kleen_closure_plus(Self::new_by_string("u", None, false)),
            Self::new_by_string("{", None, false),
        ]);
        let hex_digits = Self::concatenate(vec![
            Self::_hex_digit(), Self::_hex_digit(), Self::_hex_digit(), Self::_hex_digit(),
        ]);
        let suffix = Self::new_by_string("}", None, false);

        Self::concatenate(vec![prefix, hex_digits, suffix])
    }

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
        let l_suffix = Self::non_or_one(Self::new_by_fn(Rc::new(|c| c == 'l' || c == 'L'), None, false));

        Self::concatenate(vec![decimal_numeral, l_suffix])
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