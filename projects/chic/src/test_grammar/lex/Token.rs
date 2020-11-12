#[derive(Eq, PartialEq, Debug)]
pub struct Token {
    token_type: TokenType,
    literal: String,
    skip: bool,
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str, skip: bool) -> Token {
        Token { token_type, literal: literal.to_string(), skip }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.token_type, self.literal)
    }
}
