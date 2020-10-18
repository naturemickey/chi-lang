extern crate chic;

use chic::grammar::lex::*;

#[test]
fn test_1() {
    let s = " int float ";

    let nfa = NFA::chi_nfa();

    let mut reader = ChiReader::new(nfa.clone(), s.to_string());

    let tokens = reader.the_rest_of_tokens();

    // println!("{}", &tokens);
}