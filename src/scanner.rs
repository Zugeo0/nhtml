use crate::position::ErrorDisplay;

use super::source::Source;
use super::position::Position;
use super::token::{Token, TokenType};

#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("Invalid character '{0}' at {1}")]
    InvalidCharacter(char, ErrorDisplay),

    #[error("Malformed string at {0}")]
    MalformedString(ErrorDisplay),

    #[error("Malformed HTML at {0}")]
    MalformedHTML(ErrorDisplay),
}

pub struct Scanner<'a> {
    src: &'a str,
    pos: Position,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            pos: Position::new(),
        }
    }

    pub fn scan(&mut self) -> Result<Option<Token>, ScanError> {
        let c = match self.src.get_char(&self.pos) {
            Some(c) => c,
            None => return Ok(None),
        };

        match c {
            '"'  => self.string_token(),
            '\'' => self.string_token(),
            '=' => Ok(self.token(TokenType::Equal)),
            '{' => Ok(self.token(TokenType::LeftBrace)),
            '}' => Ok(self.token(TokenType::RightBrace)),
            ';' => Ok(self.token(TokenType::Semicolon)),

            '/' if self.if_next('*') => {
                self.multiline_comment();
                self.scan()
            },

            '<' => self.html_token(),

            '\\' => {
                self.pos.extend(self.src);
                self.pos.advance(self.src);
                self.scan()
            }

            '\n' |
            '\r' |
            '\t' |
            ' ' => {
                self.pos.advance(self.src);
                self.scan()
            }

            c if Self::is_letter(c) => Ok(self.text_token()),

            _ => Err(ScanError::InvalidCharacter(c, self.pos.for_error(self.src)))
        }
    }

    fn multiline_comment(&mut self) {
        while let Some(c) = self.src.peek_next(&self.pos) {
            self.pos.extend(self.src);

            if c == '*' && matches!(self.src.peek_next(&self.pos), Some('/')) {
                self.pos.extend(self.src);
                self.pos.advance(self.src);
                break;
            }
        }
    }

    fn string_token(&mut self) -> Result<Option<Token>, ScanError> {
        let delimeter = self.src.get_char(&self.pos)
            .expect("Scanner::string_token cannot be called with no current character");
        self.pos.extend(self.src);
        self.extend_while(|c| c != delimeter);

        let next = self.src.peek_next(&self.pos);
        
        if next.is_none() || next.unwrap() != delimeter {
            return Err(ScanError::MalformedString(self.pos.for_error(self.src)));
        }

        self.pos.extend(self.src);
        Ok(self.token(TokenType::String))
    }

    fn html_token(&mut self) -> Result<Option<Token>, ScanError> {
        self.pos.extend(self.src);

        let mut depth = 0;
        while let Some(c) = self.src.peek_next(&self.pos) {
            if c == '<' {
                depth += 1;
            }

            if c == '>' {
                if depth == 0 {
                    break;
                }

                depth -= 1;
            }
            self.pos.extend(self.src);
        }

        if !matches!(self.src.peek_next(&self.pos), Some('>')) {
            return Err(ScanError::MalformedHTML(self.pos.for_error(self.src)));
        }

        self.pos.extend(self.src);
        Ok(self.token(TokenType::Html))
    }

    fn text_token(&mut self) -> Option<Token> {
        self.extend_while(Self::is_letter_or_digit);
        self.token(TokenType::Text)
    }

    fn extend_while<F: Fn(char) -> bool>(&mut self, func: F) {
        while let Some(c) = self.src.peek_next(&self.pos) {
            if !func(c) {
                break;
            }
            self.pos.extend(self.src);
        }
    }

    fn if_next(&mut self, c: char) -> bool {
        match self.src.peek_next(&self.pos) {
            Some(match_c) if match_c == c => {
                self.pos.extend(self.src);
                true
            },
            _ => false,
        }
    }

    pub fn pos_error(&self, pos: &Position) -> ErrorDisplay {
        pos.for_error(self.src)
    } 

    pub fn pos(&self) -> Position {
        self.pos
    }

    fn token(&mut self, ty: TokenType) -> Option<Token> {
        let token = Token {
            ty,
            lexeme: self.src.get_str(&self.pos).to_owned(),
            pos: self.pos,
        };

        self.pos.advance(self.src);
        Some(token)
    }

    fn is_letter(c: char) -> bool {
        c == '_' || c == '-' || c.is_ascii_alphabetic()
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_letter_or_digit(c: char) -> bool {
        Self::is_letter(c) || Self::is_digit(c)
    }
}
