#[derive(Debug, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,

    // 식별자 + 리터럴
    IDENT,
    INT,

    // 연산자
    ASSIGN,
    PLUS,

    // 구분자
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // 예약어
    FUNCTION,
    LET,
}
