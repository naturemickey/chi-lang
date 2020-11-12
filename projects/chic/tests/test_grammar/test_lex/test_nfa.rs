#[test]
fn test_1() {
    let s = "public class A {\
    }\
    \
    fun main(args : string[]) {\
        int a = 0;
    }";

    let nfa = NFA::chi_nfa();

    // (*nfa).print();

    let mut reader = ChiReader::new(nfa.clone(), s);

    let tokens = reader.the_rest_of_tokens();

    let vec1 = vec![
        Token::new(TokenType::INT, "int", false),
//        Token::new(TokenType::WS, " ", false),
        Token::new(TokenType::FLOAT, "float", false),
    ];
    let tokens_comp = vec1;

    for token in &tokens {
        println!("{}", token.to_string());
    }

    assert_eq!(tokens, tokens_comp)
    // println!("{}", &tokens);
}