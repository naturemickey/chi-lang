extern crate chic;

use std::rc::Rc;

use chic::test_grammar::lex::{ChiReader, NFA};
use chic::test_grammar::lex::TokenType::{FLOAT, INT};

fn main() {
    f3();
}

#[allow(dead_code)]
fn f3() {
    let nfa = Rc::new(NFA::_identifier());

    let mut reader = ChiReader::new(nfa, "A");

    for token in reader.the_rest_of_tokens() {
        println!("{}", token);
    }
}

#[allow(dead_code)]
fn f1() {
    let _int = NFA::new_by_string("int", Some(INT), false);
    let _float = NFA::new_by_string("float", Some(FLOAT), false);

    println!("{}", _int.to_string());
    println!("{}", _float.to_string());

    let nfa = NFA::alternate(vec![_int, _float]);
    println!("{}", nfa.to_string());
}

#[allow(dead_code)]
fn f2() {
    let nfa = NFA::chi_nfa();

    println!("{}", nfa.to_string());
}
