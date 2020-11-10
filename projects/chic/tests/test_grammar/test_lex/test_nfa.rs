
#[test]
fn test_1() {
    let s = "int float";

    let nfa = NFA::chi_nfa();

    // (*nfa).print();

    let mut reader = ChiReader::new(nfa.clone(), s.to_string());

    let tokens = reader.the_rest_of_tokens();

    let tokens_comp = vec![
        Token::new(TokenType::INT, "int"),
        Token::new(TokenType::WS, " "),
        Token::new(TokenType::FLOAT, "float"),
    ];

    for token in &tokens {
        println!("{}", token.to_string());
    }

    assert_eq!(tokens, tokens_comp)
    // println!("{}", &tokens);
}