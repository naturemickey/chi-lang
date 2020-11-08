#[derive(Clone, Eq, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    // Keywords
    INT,
    FLOAT,
    BOOL,
    PUBLIC,
    PRIVATE,
    PROTECTED,
    FUNCTION,
    LET,
    MUTABLE,
    CHARACTER,
    OVERRIDE,
    TAILREC,
    CLASS,

    // Separators
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACK,
    RBRACK,
    SEMI,
    COMMA,
    DOT,

    // Operators
    ASSIGN,
    GT,
    LT,
    BANG,
    TILDE,
    QUESTION,
    COLON,
    EQUAL,
    LE,
    GE,
    NOTEQUAL,
    AND,
    OR,
    INC,
    DEC,
    ADD,
    SUB,
    MUL,
    DIV,
    BITAND,
    BITOR,
    CARET,
    MOD,
    ARROW,
    COLONCOLON,

    ADD_ASSIGN,
    SUB_ASSIGN,
    MUL_ASSIGN,
    DIV_ASSIGN,
    AND_ASSIGN,
    OR_ASSIGN,
    XOR_ASSIGN,
    MOD_ASSIGN,
    LSHIFT_ASSIGN,
    RSHIFT_ASSIGN,
    URSHIFT_ASSIGN,

    // Boolean Literals
    BooleanLiteral,

    // Integer Literals
    IntegerLiteral,

    WS,
    COMMENT,
    LINE_COMMENT,
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            // Keywords
            INT => "int",
            FLOAT => "float",
            BOOL => "bool",
            PUBLIC => "pub",
            PRIVATE => "pvt",
            PROTECTED => "prtc",
            FUNCTION => "fun",
            LET => "let",
            MUTABLE => "mut",
            CHARACTER => "char",
            OVERRIDE => "override",
            TAILREC => "tailrec",
            CLASS => "class",

            // Separators
            LPAREN => "(",
            RPAREN => ")",
            LBRACE => "{",
            RBRACE => "}",
            LBRACK => "[",
            RBRACK => "]",
            SEMI => ";",
            COMMA => ",",
            DOT => ".",


            // Operators
            ASSIGN => "=",
            GT => ">",
            LT => "<",
            BANG => "!",
            TILDE => "~",
            QUESTION => "?",
            COLON => ":",
            EQUAL => "==",
            LE => "<=",
            GE => ">=",
            NOTEQUAL => "!=",
            AND => "&&",
            OR => "||",
            INC => "++",
            DEC => "--",
            ADD => "+",
            SUB => "-",
            MUL => "*",
            DIV => "/",
            BITAND => "&",
            BITOR => "|",
            CARET => "^",
            MOD => "%",
            ARROW => "->",
            COLONCOLON => "::",

            ADD_ASSIGN => "+=",
            SUB_ASSIGN => "-=",
            MUL_ASSIGN => "*=",
            DIV_ASSIGN => "/=",
            AND_ASSIGN => "&=",
            OR_ASSIGN => "|=",
            XOR_ASSIGN => "^=",
            MOD_ASSIGN => "%=",
            LSHIFT_ASSIGN => "<<=",
            RSHIFT_ASSIGN => ">>=",
            URSHIFT_ASSIGN => ">>>=",

            BooleanLiteral => "",

            IntegerLiteral => "",

            WS => "\\s",
            COMMENT => "/* */",
            LINE_COMMENT => "//...",
        }.to_string()
    }
}