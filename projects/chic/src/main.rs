extern crate chic;

use chic::test_grammar::lex::NFA;
use chic::test_grammar::lex::TokenType::{FLOAT, INT};

fn main() {
    f3();
}

#[allow(dead_code)]
fn f3() {
    //let _ws_c = NFA::_ws();

    //println!("{}", _ws_c.to_string());
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
