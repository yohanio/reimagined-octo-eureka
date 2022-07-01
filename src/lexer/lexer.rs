use crate::token::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();

        let tok = match self.ch {
            b'=' => Token::ASSIGN,
            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b',' => Token::COMMA,
            b'+' => Token::PLUS,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b'-' => Token::MINUS,
            b'!' => Token::BANG,
            b'/' => Token::SLASH,
            b'*' => Token::ASTERISK,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return self.lookup_ident();
            }
            b'0'..=b'9' => {
                return self.read_number();
            }
            0 => Token::EOF,
            _ => Token::ILLEGAL
        };

        self.read_char();

        tok
    }

    fn lookup_ident(&mut self) -> Token {
        let position = self.position;

        loop {
            match self.ch {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    self.read_char();
                }
                _ => break
            }
        }

        let literal = &self.input[position..self.position];
        match literal {
            "fn" => Token::FUNCTION,
            "let" => Token::LET,
            _ => Token::IDENT(String::from(literal))
        }
    }

    fn read_number(&mut self) -> Token {
        let position = self.position;

        loop {
            match self.ch {
                b'0'..=b'9' => {
                    self.read_char();
                }
                _ => break
            }
        }

        let literal = &self.input[position..self.position];
        Token::INT(literal.parse::<i64>().unwrap())
    }

    fn eat_whitespace(&mut self) {
        loop {
            match self.ch {
                b' ' | b'\t' | b'\n' | b'\r' => {
                    self.read_char();
                }
                _ => break
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lexer::Lexer;
    use crate::token::token::Token;

    #[test]
    fn test_next_token() {
        // given
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
"#;
        let tests = vec![
            Token::LET,
            Token::IDENT(String::from("five")),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,

            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::INT(10),
            Token::SEMICOLON,

            Token::LET,
            Token::IDENT(String::from("add")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT(String::from("x")),
            Token::PLUS,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,

            Token::LET,
            Token::IDENT(String::from("result")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::RPAREN,
            Token::SEMICOLON,

            Token::EOF,
        ];

        // when
        let mut lexer = Lexer::new(input);

        // then
        for expect in tests {
            let tok = lexer.next_token();
            assert_eq!(expect, tok);
        }
    }

    #[test]
    fn test_extend_token() {
        let input = r#"!-/*5;
5 < 10 > 5;
"#;
        let tests = vec![
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT(5),
            Token::SEMICOLON,

            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::GT,
            Token::INT(5),
            Token::SEMICOLON,

            Token::EOF,
        ];

        // when
        let mut lexer = Lexer::new(input);

        // then
        for expect in tests {
            let tok = lexer.next_token();
            assert_eq!(expect, tok);
        }
    }

    #[test]
    fn test_new_keywords() {
        let input = r#"if (5 < 10) {
    return true;
} else {
    return false;
}"#;

        let tests = vec![
            Token::IF,
            Token::LPAREN,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::RPAREN,
            Token::LBRACE,

            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,

            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,

            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,

            Token::RBRACE,

            Token::EOF,
        ];

        // when
        let mut lexer = Lexer::new(input);

        // then
        for expect in tests {
            let tok = lexer.next_token();
            assert_eq!(expect, tok);
        }
    }
}
