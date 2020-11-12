extern crate chic;


use chic::test_grammar::lex::{ChiReader, NFA};
use chic::test_grammar::lex::TokenType::{FLOAT, INT};
use std::rc::Rc;
use std::fs::File;
use std::io::Read;

fn main() {
    f4();
}

fn f4() {
    println!("0");

    let mut file = File::open("test_cases/test_lexer.chi").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    println!("file content: {}", s);

    println!("1");
    let nfa = NFA::chi_nfa();
    // let nfa = Rc::new(NFA::_integer_literal());

    // println!("nfa : {}", nfa);

    println!("2");
    // (*nfa).print();

    let mut reader = ChiReader::new(nfa.clone(), &s);

    println!("3");
    let tokens = reader.the_rest_of_tokens();


    println!("4");

    for token in &tokens {
        println!("{}", token.to_string());
    }
}

#[allow(dead_code)]
fn f3() {
    // let nfa = Rc::new(NFA::_floating_point_literal());
    let nfa = NFA::chi_nfa();

    let mut reader = ChiReader::new(nfa, "0D");

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
