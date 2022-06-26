use crate::token::token::Token;

pub struct Lexer {}

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn next_token(&self) -> Token {
        Token::ILLEGAL
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lexer::Lexer;
    use crate::token::token::Token;

    #[test]
    fn test_next_token() {
        // given
        let input = r#"=+(){},;"#;
        let tests = vec![
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
            Token::EOF
        ];

        // when
        let mut lexer = Lexer::new();

        // then
        for expect in tests {
            let tok = lexer.next_token();
            assert_eq!(expect, tok);
        }
    }
}
