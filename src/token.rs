use super::position::Position;

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub pos: Position,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Text,
    String,
    Equal,
    LeftBrace,
    RightBrace,
    Html,
    Semicolon,
}