#[test]
fn test_1() {
    println!("0");

    let mut file = File::open("test_cases/test_lexera.chi").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    println!("1");
    let nfa = NFA::chi_nfa();

    println!("2");
    // (*nfa).print();

    let mut reader = ChiReader::new(nfa.clone(), &s);

    println!("3");
    let tokens = reader.the_rest_of_tokens();


    println!("4");
    let vec1 = vec![
        Token::new(TokenType::INT, "int", false),
//        Token::new(TokenType::WS, " ", false),
        Token::new(TokenType::FLOAT, "float", false),
    ];
    let tokens_comp = vec1;

    for token in &tokens {
        println!("{}", token.to_string());
    }
    println!("0");

    let mut file = File::open("test_cases/test_lexer.chi").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    println!("1");
    let nfa = NFA::chi_nfa();

    println!("2");
    // (*nfa).print();

    let mut reader = ChiReader::new(nfa.clone(), &s);

    println!("3");
    let tokens = reader.the_rest_of_tokens();


    println!("4");
    let vec1 = vec![
        Token::new(TokenType::INT, "int", false),
//        Token::new(TokenType::WS, " ", false),
        Token::new(TokenType::FLOAT, "float", false),
    ];
    let tokens_comp = vec1;

    for token in &tokens {
        println!("{}", token.to_string());
    }

    // assert_eq!(tokens, tokens_comp)
    // println!("{}", &tokens);
}