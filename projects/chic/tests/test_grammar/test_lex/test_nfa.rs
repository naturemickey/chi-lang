#[test]
fn test_1() {
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
        Token::new(TokenType::Identifier, "public", false),
        Token::new(TokenType::Identifier, "class", false),
        Token::new(TokenType::Identifier, "A", false),
        Token::new(TokenType::LBRACE, "{", false),
        Token::new(TokenType::RBRACE, "}", false),
        Token::new(TokenType::Identifier, "fun", false),
        Token::new(TokenType::Identifier, "main", false),
        Token::new(TokenType::LPAREN, "(", false),
        Token::new(TokenType::Identifier, "args", false),
        Token::new(TokenType::COLON, ":", false),
        Token::new(TokenType::Identifier, "string", false),
        Token::new(TokenType::LBRACK, "[", false),
        Token::new(TokenType::RBRACK, "]", false),
        Token::new(TokenType::RPAREN, ")", false),
        Token::new(TokenType::LBRACE, "{", false),
        Token::new(TokenType::Identifier, "int", false),
        Token::new(TokenType::Identifier, "a", false),
        Token::new(TokenType::ASSIGN, "=", false),
        Token::new(TokenType::IntegerLiteral, "1L", false),
        Token::new(TokenType::SEMI, ";", false),
        Token::new(TokenType::Identifier, "let", false),
        Token::new(TokenType::Identifier, "mut", false),
        Token::new(TokenType::Identifier, "c", false),
        Token::new(TokenType::ASSIGN, "=", false),
        Token::new(TokenType::CHARACTER, "'c'", false),
        Token::new(TokenType::SEMI, ";", false),
        Token::new(TokenType::Identifier, "let", false),
        Token::new(TokenType::Identifier, "f", false),
        Token::new(TokenType::COLON, ":", false),
        Token::new(TokenType::Identifier, "float", false),
        Token::new(TokenType::ASSIGN, "=", false),
        Token::new(TokenType::FloatingPointLiteral, "0.1D", false),
        Token::new(TokenType::SEMI, ";", false),
        Token::new(TokenType::Identifier, "let", false),
        Token::new(TokenType::Identifier, "s", false),
        Token::new(TokenType::ASSIGN, "=", false),
        Token::new(TokenType::StringLiteral, "\"abc\\u{ABCD}\"", false),
        Token::new(TokenType::SEMI, ";", false),
        Token::new(TokenType::Identifier, "let", false),
        Token::new(TokenType::Identifier, "f1", false),
        Token::new(TokenType::ASSIGN, "=", false),
        Token::new(TokenType::FloatingPointLiteral, "1.23e-5", false),
        Token::new(TokenType::SEMI, ";", false),
        Token::new(TokenType::Identifier, "if", false),
        Token::new(TokenType::LPAREN, "(", false),
        Token::new(TokenType::Identifier, "a", false),
        Token::new(TokenType::GE, ">=", false),
        Token::new(TokenType::Identifier, "b", false),
        Token::new(TokenType::RPAREN, ")", false),
        Token::new(TokenType::RBRACE, "}", false),
    ];
    let tokens_comp = vec1;

    for token in &tokens {
        println!("{}", token.to_string());
    }

    assert_eq!(tokens, tokens_comp)
    // println!("{}", &tokens);
}