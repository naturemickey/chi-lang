use crate::grammar::lex::StateType::NORMAL;
use std::str::Chars;
use std::rc::Rc;

include!("NFA.rs");
include!("State.rs");
include!("StateNext.rs");
include!("StateType.rs");
include!("TokenType.rs");
include!("Token.rs");