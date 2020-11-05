#[derive(Eq, PartialEq, Debug)]
pub struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Token {
        Token { token_type, literal: literal.to_string() }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        let mut s = String::new();

        write!(s, "({}, {})", self.token_type.to_string(), self.literal);

        s
    }
}
