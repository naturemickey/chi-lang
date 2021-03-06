#[derive(Clone, Eq, PartialEq, Debug, Copy)]
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
    OBJECT,

    IMPLEMENTS,
    PACKAGE,
    DEFINE,
    DATA,
    CATCH,
    CONST,
    CONTINUE,
    DEFAULT,
    DO,
    IF,
    ELSE,
    ENUM,
    EXTENDS,
    FINAL,
    FINALLY,
    FOR,
    GOTO,
    IMPORT,
    INSTANCEOF,
    INTERFACE,
    LONG,
    NATIVE,
    NEW,
    RETURN,
    SHORT,
    STATIC,
    STRICTFP,
    SUPER,
    SWITCH,
    SYNCHRONIZED,
    THIS,
    THROW,
    THROWS,
    TRANSIENT,
    TRY,
    VOID,
    VOLATILE,
    WHILE,
    TRAIT,
    WITH,
    USE,
    SEALED,
    SELF,
    MATCH,
    ABSTRACT,
    ASSERT,
    BREAK,
    BYTE,
    CASE,
    VAL,
    VAR,
    TYPE,
    LAZY,
    IMPLICIT,
    YIELD,

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

    BooleanLiteral,
    IntegerLiteral,
    FloatingPointLiteral,
    CharacterLiteral,
    StringLiteral,
    Identifier,

    WS,
    COMMENT,
    LINE_COMMENT,
}

impl TokenType {
    pub fn get_level(self) -> i32 {
        self as i32
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
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
            OBJECT => "OBJECT",
            IMPLEMENTS => "IMPLEMENTS",
            PACKAGE => "PACKAGE",
            DEFINE => "DEFINE",
            DATA => "DATA",
            CATCH => "CATCH",
            CONST => "CONST",
            CONTINUE => "CONTINUE",
            DEFAULT => "DEFAULT",
            DO => "DO",
            IF => "IF",
            ELSE => "ELSE",
            ENUM => "ENUM",
            EXTENDS => "EXTENDS",
            FINAL => "FINAL",
            FINALLY => "FINALLY",
            FOR => "FOR",
            GOTO => "GOTO",
            IMPORT => "IMPORT",
            INSTANCEOF => "INSTANCEOF",
            INTERFACE => "INTERFACE",
            LONG => "LONG",
            NATIVE => "NATIVE",
            NEW => "NEW",
            RETURN => "RETURN",
            SHORT => "SHORT",
            STATIC => "STATIC",
            STRICTFP => "STRICTFP",
            SUPER => "SUPER",
            SWITCH => "SWITCH",
            SYNCHRONIZED => "SYNCHRONIZED",
            THIS => "THIS",
            THROW => "THROW",
            THROWS => "THROWS",
            TRANSIENT => "TRANSIENT",
            TRY => "TRY",
            VOID => "VOID",
            VOLATILE => "VOLATILE",
            WHILE => "WHILE",
            TRAIT => "TRAIT",
            WITH => "WITH",
            USE => "USE",
            SEALED => "SEALED",
            SELF => "SELF",
            MATCH => "MATCH",
            ABSTRACT => "ABSTRACT",
            ASSERT => "ASSERT",
            BREAK => "BREAK",
            BYTE => "BYTE",
            CASE => "CASE",
            VAL => "VAL",
            VAR => "VAR",
            TYPE => "TYPE",
            LAZY => "LAZY",
            IMPLICIT => "IMPLICIT",
            YIELD => "YIELD",

            // Separators
            LPAREN => "LPAREN",
            RPAREN => "RPAREN",
            LBRACE => "LBRACE",
            RBRACE => "RBRACE",
            LBRACK => "LBRACK",
            RBRACK => "RBRACK",
            SEMI => "SEMI",
            COMMA => "COMMA",
            DOT => "DOT",

            // Operators
            ASSIGN => "ASSIGN",
            GT => "GT",
            LT => "LT",
            BANG => "BANG",
            TILDE => "TILDE",
            QUESTION => "QUESTION",
            COLON => "COLON",
            EQUAL => "EQUAL",
            LE => "LE",
            GE => "GE",
            NOTEQUAL => "NOTEQUAL",
            AND => "AND",
            OR => "OR",
            INC => "INC",
            DEC => "DEC",
            ADD => "ADD",
            SUB => "SUB",
            MUL => "MUL",
            DIV => "DIV",
            BITAND => "BITAND",
            BITOR => "BITOR",
            CARET => "CARET",
            MOD => "MOD",
            ARROW => "ARROW",
            COLONCOLON => "COLONCOLON",
            ADD_ASSIGN => "ADD_ASSIGN",
            SUB_ASSIGN => "SUB_ASSIGN",
            MUL_ASSIGN => "MUL_ASSIGN",
            DIV_ASSIGN => "DIV_ASSIGN",
            AND_ASSIGN => "AND_ASSIGN",
            OR_ASSIGN => "OR_ASSIGN",
            XOR_ASSIGN => "XOR_ASSIGN",
            MOD_ASSIGN => "MOD_ASSIGN",
            LSHIFT_ASSIGN => "LSHIFT_ASSIGN",
            RSHIFT_ASSIGN => "RSHIFT_ASSIGN",
            URSHIFT_ASSIGN => "URSHIFT_ASSIGN",
            BooleanLiteral => "BooleanLiteral",
            IntegerLiteral => "IntegerLiteral",
            FloatingPointLiteral => "FloatingPointLiteral",
            CharacterLiteral => "CharacterLiteral",
            StringLiteral => "StringLiteral",
            Identifier => "Identifier",
            WS => "WS",
            COMMENT => "COMMENT",
            LINE_COMMENT => "LINE_COMMENT",
        };
        write!(f, "{}", s)
    }
}