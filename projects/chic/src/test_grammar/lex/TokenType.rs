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

    // Character Literals
    CharacterLiteral,

    WS,
    COMMENT,
    LINE_COMMENT,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
      let s =  match self {
            // Keywords
            INT => "INT",
            FLOAT => "FLOAT",
            BOOL => "BOOL",
            PUBLIC => "PUBLIC",
            PRIVATE => "PRIVATE",
            PROTECTED => "PROTECTED",
            FUNCTION => "FUNCTION",
            LET => "LET",
            MUTABLE => "MUTABLE",
            CHARACTER => "CHARACTER",
            OVERRIDE => "OVERRIDE",
            TAILREC => "TAILREC",
            CLASS => "CLASS",

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

            BooleanLiteral => "BooleanLiteral",

            IntegerLiteral => "IntegerLiteral",

            CharacterLiteral => "CharacterLiteral",

            WS => "WS",
            COMMENT => "/* */",
            LINE_COMMENT => "//...",
        };
        write!(f, "{}", s)
    }
}