#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TokenType {
    INT,
    FLOAT,
    WS,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            INT => "INT",
            FLOAT => "FLOAT",
            WS => "WS",
        })
    }
}
