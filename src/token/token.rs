#[derive(Debug, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,

    // 식별자 + 리터럴
    IDENT(String),
    INT(i64),

    // 연산자
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,

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
