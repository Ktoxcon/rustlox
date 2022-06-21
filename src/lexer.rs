use crate::error::CompilationError;
use crate::token::{Token, TokenType};
use std::vec::Vec;

pub struct Lexer {
    source_code: String,
    line: usize,
    start_position: usize,
    current_position: usize,
    pub has_error: bool,
}

impl Lexer {
    pub fn new(source_code: String) -> Self {
        Self {
            source_code,
            start_position: 0,
            current_position: 0,
            line: 1,
            has_error: false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_position >= self.source_code.len()
    }

    fn get_next_char(&mut self) -> Option<char> {
        let next_char_index = usize::from(self.current_position);

        self.source_code.chars().nth(next_char_index + 1)
    }

    fn get_current_char(&self) -> Option<char> {
        self.source_code
            .chars()
            .nth(usize::from(self.current_position))
    }

    fn advance(&mut self) {
        self.current_position += 1;
    }

    fn scan_token(&mut self, errors: &mut Vec<CompilationError>, tokens: &mut Vec<Token>) {
        let current_char = self.get_current_char().unwrap();

        match current_char {
            '{' => {
                tokens.push(Token {
                    lexeme: String::from(current_char),
                    token_type: TokenType::LeftCurlyBracket,
                    literal: None,
                    line: usize::from(self.line),
                });
                self.advance();
            }
            '}' => {
                tokens.push(Token {
                    lexeme: String::from(current_char),
                    token_type: TokenType::RightCurlyBracket,
                    literal: None,
                    line: usize::from(self.line),
                });
                self.advance();
            }
            '[' => {
                tokens.push(Token {
                    lexeme: String::from(current_char),
                    token_type: TokenType::LeftSquareBracket,
                    literal: None,
                    line: usize::from(self.line),
                });
                self.advance();
            }
            ']' => {
                tokens.push(Token {
                    lexeme: String::from(current_char),
                    token_type: TokenType::RightSquareBracket,
                    literal: None,
                    line: usize::from(self.line),
                });
                self.advance();
            }
            '(' => {
                tokens.push(Token {
                    lexeme: String::from(current_char),
                    token_type: TokenType::LeftParenthesis,
                    literal: None,
                    line: usize::from(self.line),
                });
                self.advance();
            }
            ')' => {
                tokens.push(Token {
                    lexeme: String::from(current_char),
                    token_type: TokenType::RightParenthesis,
                    literal: None,
                    line: usize::from(self.line),
                });
                self.advance();
            }
            ':' => {
                tokens.push(Token {
                    lexeme: String::from(current_char),
                    token_type: TokenType::Colon,
                    literal: None,
                    line: usize::from(self.line),
                });
                self.advance();
            }
            ';' => {
                tokens.push(Token {
                    lexeme: String::from(current_char),
                    token_type: TokenType::Semicolon,
                    literal: None,
                    line: usize::from(self.line),
                });
                self.advance();
            }
            '.' => {
                tokens.push(Token {
                    lexeme: String::from(current_char),
                    token_type: TokenType::Dot,
                    literal: None,
                    line: usize::from(self.line),
                });
                self.advance();
            }
            ',' => {
                tokens.push(Token {
                    lexeme: String::from(current_char),
                    token_type: TokenType::Coma,
                    literal: None,
                    line: usize::from(self.line),
                });
                self.advance();
            }
            '?' => {
                tokens.push(Token {
                    lexeme: String::from(current_char),
                    token_type: TokenType::Question,
                    literal: None,
                    line: usize::from(self.line),
                });
                self.advance();
            }
            '=' => {
                let next_char = self.get_next_char();
                match next_char {
                    Some('=') => {
                        tokens.push(Token {
                            lexeme: format!("{}{}", current_char, next_char.unwrap()),
                            token_type: TokenType::DoubleEqual,
                            literal: None,
                            line: usize::from(self.line),
                        });
                    }

                    None => (),
                    _ => {
                        tokens.push(Token {
                            lexeme: String::from(current_char),
                            token_type: TokenType::Equal,
                            literal: None,
                            line: usize::from(self.line),
                        });
                    }
                }

                self.advance();
            }
            '>' => {
                let next_char = self.get_next_char();
                match next_char {
                    Some('=') => {
                        tokens.push(Token {
                            lexeme: format!("{}{}", current_char, next_char.unwrap()),
                            token_type: TokenType::GreaterEqual,
                            literal: None,
                            line: usize::from(self.line),
                        });
                    }

                    None => (),
                    _ => {
                        tokens.push(Token {
                            lexeme: String::from(current_char),
                            token_type: TokenType::Greater,
                            literal: None,
                            line: usize::from(self.line),
                        });
                    }
                }

                self.advance();
            }
            '<' => {
                let next_char = self.get_next_char();
                match next_char {
                    Some('=') => {
                        tokens.push(Token {
                            lexeme: format!("{}{}", current_char, next_char.unwrap()),
                            token_type: TokenType::LessEqual,
                            literal: None,
                            line: usize::from(self.line),
                        });
                    }
                    None => (),
                    _ => {
                        tokens.push(Token {
                            lexeme: String::from(current_char),
                            token_type: TokenType::Less,
                            literal: None,
                            line: usize::from(self.line),
                        });
                    }
                }
                self.advance();
            }
            '+' => {
                tokens.push(Token {
                    lexeme: String::from(current_char),
                    token_type: TokenType::Plus,
                    literal: None,
                    line: usize::from(self.line),
                });
                self.advance();
            }
            '-' => {
                tokens.push(Token {
                    lexeme: String::from(current_char),
                    token_type: TokenType::Minus,
                    literal: None,
                    line: usize::from(self.line),
                });
                self.advance();
            }
            '*' => {
                tokens.push(Token {
                    lexeme: String::from(current_char),
                    token_type: TokenType::Star,
                    literal: None,
                    line: usize::from(self.line),
                });
                self.advance();
            }
            '/' => {
                let next_char = self.get_next_char();

                match next_char {
                    Some('/') => {
                        while let Some(comment_char) = self.get_next_char() {
                            if comment_char == '\n' || self.is_at_end() {
                                break;
                            }

                            self.advance()
                        }
                    }
                    _ => {
                        tokens.push(Token {
                            lexeme: String::from(current_char),
                            token_type: TokenType::Slash,
                            literal: None,
                            line: usize::from(self.line),
                        });
                        self.advance();
                    }
                }
            }

            '\n' => {
                self.line += 1;
                self.advance();
            }

            ' ' | '\r' | '\t' => self.current_position += 1,
            _ => {
                errors.push(CompilationError {
                    line: self.line,
                    message: format!("Unexpected char: '{}' at line {}", current_char, self.line),
                });
                self.has_error = true;
                self.current_position += 1
            }
        }
    }

    pub fn scan_tokens(&mut self) -> (Option<Vec<CompilationError>>, Vec<Token>) {
        let mut tokens = Vec::new();
        let mut errors: Vec<CompilationError> = Vec::new();

        while !self.is_at_end() {
            self.start_position = self.current_position;
            self.scan_token(&mut errors, &mut tokens);
        }

        tokens.push(Token {
            lexeme: String::from(""),
            line: self.line,
            literal: None,
            token_type: TokenType::Eof,
        });

        (Some(errors), tokens)
    }
}
