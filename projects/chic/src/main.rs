extern crate chic;

use chic::grammar::lex::NFA;
use chic::grammar::lex::TokenType::{INT, FLOAT};

fn main() {
    let _int = NFA::new_by_string("int", Some(INT), false);
    let _float = NFA::new_by_string("float", Some(FLOAT), false);

    println!("{}", _int.to_string());
    println!("{}", _float.to_string());

    let nfa = NFA::alternate( vec![_int, _float]);
    println!("{}", nfa.to_string());
}
