#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TokenType {
    INT,
    FLOAT,
    WS,
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
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            INT => "INT",
            FLOAT => "FLOAT",
            WS => "FLOAT",
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
        }.to_string()
    }
}