#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TokenType {
    INT,
    FLOAT,
    WS,
}

// impl Display for TokenType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", match self {
//             INT => "INT",
//             FLOAT => "FLOAT",
//             WS => "WS",
//         })
//     }
// }

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            INT => "INT",
            FLOAT => "FLOAT",
            WS => "FLOAT",
        }.to_string()
    }
}