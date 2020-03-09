use crate::scanner::file::SourceController;
use crate::scanner::token::{Token, TokenType};

pub struct Scanner {
    source: SourceController,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: SourceController::new(source),
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.source.skip_whitespaces();

        self.source.set_start_to_next_token();

        if self.source.is_eof() {
            return self.make_token(TokenType::EOF);
        }

        let char = self.source.advance();

        return match char {
            '(' => self.make_token(TokenType::LEFT_PAREN),
            ')' => self.make_token(TokenType::RIGHT_PAREN),
            '{' => self.make_token(TokenType::LEFT_BRACE),
            '}' => self.make_token(TokenType::RIGHT_BRACE),
            ',' => self.make_token(TokenType::COMMA),
            '.' => self.make_token(TokenType::DOT),
            '-' => self.make_token(TokenType::MINUS),
            '+' => self.make_token(TokenType::PLUS),
            ';' => self.make_token(TokenType::SEMICOLON),
            '*' => self.make_token(TokenType::STAR),
            '!' => {
                (if self.source.advance_match('=') {
                    self.make_token(TokenType::BANG_EQUAL)
                } else {
                    self.make_token(TokenType::BANG)
                })
            }
            '=' => {
                (if self.source.advance_match('=') {
                    self.make_token(TokenType::EQUAL_EQUAL)
                } else {
                    self.make_token(TokenType::EQUAL)
                })
            }
            '<' => {
                (if self.source.advance_match('=') {
                    self.make_token(TokenType::LESS_EQUAL)
                } else {
                    self.make_token(TokenType::LESS)
                })
            }
            '>' => {
                (if self.source.advance_match('=') {
                    self.make_token(TokenType::GREATER_EQUAL)
                } else {
                    self.make_token(TokenType::GREATER)
                })
            }
            '"' => self.string(),
            '0'...'9' => self.number(),
            'a'...'z' | 'A'...'Z' | '_' => self.identifier(),
            _ => self.make_error("Unexpected character."),
        };
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            self.source.extract_from_start(),
            self.source.line,
        )
    }

    fn make_error(&mut self, message: &str) -> Token {
        Token::error(message, self.source.line)
    }

    fn string(&mut self) -> Token {
        while self.source.query_current() != '"' && !self.source.is_eof() {
            if self.source.query_current() == '\n' {
                self.source.line += 1;
            }
            self.source.advance();
        }

        if self.source.is_eof() {
            self.make_error("Unterminated string")
        } else {
            self.source.advance();
            self.make_token(TokenType::STRING)
        }
    }

    fn number(&mut self) -> Token {
        while self.source.query_current().is_numeric() {
            self.source.advance();
        }

        if self.source.query_current() == '.' && self.source.query_next().is_numeric() {
            self.source.advance();
            while self.source.query_current().is_numeric() {
                self.source.advance();
            }
        }

        self.make_token(TokenType::NUMBER)
    }

    fn identifier(&mut self) -> Token {
        while self.source.query_current().is_alphanumeric() {
            self.source.advance();
        }

        let token_type = self.identifier_type();

        self.make_token(token_type)
    }

    fn identifier_type(&self) -> TokenType {
        let name = self.source.extract_from_start();
        let mut chars = name.chars();

        match chars.next().unwrap() {
            'a' => is_a_keyword(&name[1..], "nd", TokenType::AND),
            'c' => is_a_keyword(&name[1..], "lass", TokenType::CLASS),
            'e' => is_a_keyword(&name[1..], "lse", TokenType::ELSE),
            'f' if name.len() > 1 => match chars.next().unwrap() {
                'a' => is_a_keyword(&name[2..], "lse", TokenType::FALSE),
                'o' => is_a_keyword(&name[2..], "r", TokenType::FOR),
                'u' => is_a_keyword(&name[2..], "n", TokenType::FUN),
                _ => TokenType::IDENTIFIER,
            },
            'i' => is_a_keyword(&name[1..], "f", TokenType::IF),
            'n' => is_a_keyword(&name[1..], "il", TokenType::NIL),
            'o' => is_a_keyword(&name[1..], "r", TokenType::OR),
            'p' => is_a_keyword(&name[1..], "rint", TokenType::PRINT),
            'r' => is_a_keyword(&name[1..], "eturn", TokenType::RETURN),
            's' => is_a_keyword(&name[1..], "uper", TokenType::SUPER),
            't' if name.len() > 1 => match chars.next().unwrap() {
                'h' => is_a_keyword(&name[2..], "is", TokenType::THIS),
                'r' => is_a_keyword(&name[2..], "ue", TokenType::TRUE),
                _ => TokenType::IDENTIFIER,
            },
            'v' => is_a_keyword(&name[1..], "ar", TokenType::VAR),
            'w' => is_a_keyword(&name[1..], "hile", TokenType::WHILE),
            _ => TokenType::IDENTIFIER,
        }
    }
}

fn is_a_keyword(a: &str, b: &str, token_type: TokenType) -> TokenType {
    if a == b {
        token_type
    } else {
        TokenType::IDENTIFIER
    }
}

#[cfg(test)]
mod tests {
    use super::Scanner;
    use crate::scanner::token::TokenType;

    #[test]
    fn test_identifier_trie() {
        let mut scanner = Scanner::new("this NON_RESERVED");
        let reserved = scanner.scan_token();
        let non_reserved = scanner.scan_token();

        assert_eq!(TokenType::THIS, reserved.token_type);
        assert_eq!(TokenType::IDENTIFIER, non_reserved.token_type);
    }
}
