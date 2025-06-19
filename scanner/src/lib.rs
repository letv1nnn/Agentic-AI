use std::collections::HashMap;


#[derive(Clone)]
#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Minus, Plus,
    RightShift, LeftShift,
    Semicolon, Slash, Star,
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    Identifier, Str, Number,
    And, Else, False,
    Fun, If, Noth, For,
    Print, Return, Or,
    Var, While, True,
    Error, Eof
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("while".to_string(), TokenType::While);
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("noth".to_string(), TokenType::Noth);
        keywords.insert("fun".to_string(), TokenType::Fun);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);

        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        match c {
            '+' => self.make_token(TokenType::Plus),
            '-' => self.make_token(TokenType::Minus),
            '*' => self.make_token(TokenType::Star),
            '/' => self.make_token(TokenType::Slash),
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ',' => self.make_token(TokenType::Comma),
            ';' => self.make_token(TokenType::Semicolon),
            '"' => self.string(),
            c if c.is_ascii_digit() => self.numbers(),
            c if c.is_ascii_alphabetic() => self.identifier(),

            // need to handle tokens with length more than one like < or <=, etc.
            '=' => {
                let exp = self.match_char('=');
                match exp {
                    true => self.make_token(TokenType::EqualEqual),
                    false => self.make_token(TokenType::Equal),
                }
            },
            '!' => {
                let exp = self.match_char('=');
                match exp {
                    true => self.make_token(TokenType::BangEqual),
                    false => self.make_token(TokenType::Bang),
                }
            },
            '<' => {
                // introducing left shifting here
                let exp = self.source.chars().nth(self.current).unwrap();
                self.current += 1;
                match exp {
                    '=' => self.make_token(TokenType::LessEqual),
                    '<' => self.make_token(TokenType::LeftShift),
                    _ => self.make_token(TokenType::Less),
                }
            },
            '>' => {
                let exp = self.source.chars().nth(self.current).unwrap();
                self.current += 1;
                match exp {
                    '=' => self.make_token(TokenType::GreaterEqual),
                    '>' => self.make_token(TokenType::RightShift),
                    _ => self.make_token(TokenType::Greater),
                }
            }

            _ => self.make_token(TokenType::Error),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }
        self.current += 1;
        true
    }

    fn string(&mut self) -> Token {
        while let Some(c) = self.source.chars().nth(self.current) {
            if c == '"' {
                self.advance();
                return self.make_token(TokenType::Str);
            }
            if c == '$' {
                if self.match_char('{') {
                    self.identifier();
                }
                if self.advance() != '}' {
                    return self.make_token(TokenType::Error);
                }
            }
            if c == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        self.make_token(TokenType::Error)
    }

    fn numbers(&mut self) -> Token {
        while let Some(c) = self.source.chars().nth(self.current) {
            if !c.is_ascii_digit() {
                break;
            }
            self.advance();
        }
        // Look for fractional part
        if let Some('.') = self.source.chars().nth(self.current) {
            if let Some(c) = self.source.chars().nth(self.current + 1) {
                if c.is_ascii_digit() {
                    self.advance();
                    while let Some(c) = self.source.chars().nth(self.current) {
                        if !c.is_ascii_digit() {
                           break;
                        }
                        self.advance();
                    }
                }
            }
        }
        self.make_token(TokenType::Number)
    }

    // need to  also handle the keywords recognition, like for/false or the variable
    fn identifier(&mut self) -> Token {
        while let Some(c) = self.source.chars().nth(self.current) {
            if !c.is_ascii_alphabetic() && c != '_' {
                break;
            }
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        match self.keywords.get(text) {
            Some(token_type) => self.make_token(token_type.clone()),
            None => self.make_token(TokenType::Identifier),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let curr = self.source.chars().nth(self.current);
            match curr {
                Some(' ') => self.advance(),
                Some('\t') | Some('\r') => self.advance(),
                Some('\n') => {
                    self.line += 1;
                    self.advance()
                },
                Some('/') => {
                    if let Some('/') = self.source.chars().nth(self.current + 1) {
                        // A comment goes until the end of the line
                        while let Some(c) = self.source.chars().nth(self.current) {
                            if c == '\n' {
                                break;
                            }
                            self.advance();
                        } return
                    } else {
                        break;
                    }
                },
                _ => return,
            };
        }
    }
    // returns a token with a specific type and
    // start____current slice of the string code
    fn make_token(&self, token_type: TokenType) -> Token {
        let lexeme = self.source[self.start..self.current].to_string();
        Token {
            token_type,
            lexeme,
            line: self.line,
        }
    }
}


// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_expression() {
        let source = String::from("print(1 + 2 / 86)");
        let mut scanner = Scanner::new(source);

        let print_token = scanner.scan_token();
        assert_eq!(print_token.token_type, TokenType::Print);

        let left_paren_token = scanner.scan_token();
        assert_eq!(left_paren_token.token_type, TokenType::LeftParen);

        let number_token = scanner.scan_token();
        assert_eq!(number_token.token_type, TokenType::Number);

        let plus_token = scanner.scan_token();
        assert_eq!(plus_token.token_type, TokenType::Plus);

        let number_token = scanner.scan_token();
        assert_eq!(number_token.token_type, TokenType::Number);

        let slash_token = scanner.scan_token();
        assert_eq!(slash_token.token_type, TokenType::Slash);

        let number_token = scanner.scan_token();
        assert_eq!(number_token.token_type, TokenType::Number);

        let right_paren_token = scanner.scan_token();
        assert_eq!(right_paren_token.token_type, TokenType::RightParen);
    }

    #[test]
    fn test_for_loop() {
        let source = String::from("for (var i = 0; i < 10; i = i + 1) {\n\tprint(i)\n}");
        let mut scanner = Scanner::new(source);

        let for_token = scanner.scan_token();
        assert_eq!(for_token.token_type, TokenType::For);
        assert_eq!(for_token.lexeme, "for");

        let left_paren = scanner.scan_token();
        assert_eq!(left_paren.token_type, TokenType::LeftParen);

        let var_token = scanner.scan_token();
        assert_eq!(var_token.token_type, TokenType::Var);
        assert_eq!(var_token.lexeme, "var");

        let i_token = scanner.scan_token();
        assert_eq!(i_token.token_type, TokenType::Identifier);
        assert_eq!(i_token.lexeme, "i");

        let equal_token = scanner.scan_token();
        assert_eq!(equal_token.token_type, TokenType::Equal);

        let zero_token = scanner.scan_token();
        assert_eq!(zero_token.token_type, TokenType::Number);
        assert_eq!(zero_token.lexeme, "0");

        let semicolon_token = scanner.scan_token();
        assert_eq!(semicolon_token.token_type, TokenType::Semicolon);

        let i_token = scanner.scan_token();
        assert_eq!(i_token.token_type, TokenType::Identifier);
        assert_eq!(i_token.lexeme, "i");

        let less_token = scanner.scan_token();
        assert_eq!(less_token.token_type, TokenType::Less);

        let ten_token = scanner.scan_token();
        assert_eq!(ten_token.token_type, TokenType::Number);
        assert_eq!(ten_token.lexeme, "10");

        let semicolon_token = scanner.scan_token();
        assert_eq!(semicolon_token.token_type, TokenType::Semicolon);

        let i_token = scanner.scan_token();
        assert_eq!(i_token.token_type, TokenType::Identifier);
        assert_eq!(i_token.lexeme, "i");

        let equal_token = scanner.scan_token();
        assert_eq!(equal_token.token_type, TokenType::Equal);

        let i_token = scanner.scan_token();
        assert_eq!(i_token.token_type, TokenType::Identifier);
        assert_eq!(i_token.lexeme, "i");

        let plus_token = scanner.scan_token();
        assert_eq!(plus_token.token_type, TokenType::Plus);

        let one_token = scanner.scan_token();
        assert_eq!(one_token.token_type, TokenType::Number);
        assert_eq!(one_token.lexeme, "1");

        let right_paren = scanner.scan_token();
        assert_eq!(right_paren.token_type, TokenType::RightParen);

        let left_brace = scanner.scan_token();
        assert_eq!(left_brace.token_type, TokenType::LeftBrace);

        let print_token = scanner.scan_token();
        assert_eq!(print_token.token_type, TokenType::Print);

        let left_paren = scanner.scan_token();
        assert_eq!(left_paren.token_type, TokenType::LeftParen);

        let i_token = scanner.scan_token();
        assert_eq!(i_token.token_type, TokenType::Identifier);
        assert_eq!(i_token.lexeme, "i");

        let right_paren = scanner.scan_token();
        assert_eq!(right_paren.token_type, TokenType::RightParen);

        let right_brace = scanner.scan_token();
        assert_eq!(right_brace.token_type, TokenType::RightBrace);

        let eof_token = scanner.scan_token();
        assert_eq!(eof_token.token_type, TokenType::Eof);
    }

}

