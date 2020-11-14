#[test]
fn test_token_type_i32() {
    assert_eq!(TokenType::INT as i32, 0);
    assert_eq!(TokenType::PUBLIC as i32, 3);
}