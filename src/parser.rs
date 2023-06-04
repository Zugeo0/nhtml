use crate::{token::{Token, TokenType}, scanner::{Scanner, ScanError}, source::Source};
use anyhow::Result;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unexpected token. {0}, {1:#?}")]
    UnexpectedToken(String, Option<Token>),

    #[error("Expected tag. Got {0:#?}")]
    ExpectedTag(Option<Token>),

    #[error("Expected element. Got {0:#?}")]
    ExpectedElement(Option<Token>),
}

#[derive(Debug)]
pub struct Tag {
    pub ty: String,
    pub attribs: Vec<(String, Option<String>)>,
    pub body: Vec<Element>,
}

#[derive(Debug)]
pub enum Element {
    Tag(Tag),
    Text(String),
    HTML(String),
}

pub struct Parser<'a, S: Source> {
    scanner: &'a mut Scanner<'a, S>,
    unused: Vec<Token>,
}

impl<'a, S: Source> Parser<'a, S> {
    pub fn parse(scanner: &'a mut Scanner<'a, S>) -> Result<Vec<Element>> {
        let mut parser = Parser {
            scanner,
            unused: vec![],
        };

        let mut tags = vec![];

        while let Some(tag) = parser.parse_element()? {
            tags.push(tag);
        }

        Ok(tags)
    }

    pub fn parse_element(&mut self) -> Result<Option<Element>> {
        if self.is_next(TokenType::Text) {
            let tag = self.parse_tag()?;
            Ok(Some(Element::Tag(tag)))
        } else if self.is_next(TokenType::String) {
            let string = self.take()?.unwrap();
            let value = string.lexeme[1..string.lexeme.len() - 1].to_owned();
            Ok(Some(Element::Text(value)))
        } else if self.is_next(TokenType::Html) {
            let html = self.take()?.unwrap();
            Ok(Some(Element::HTML(html.lexeme)))
        } else {
            Ok(None)
        }
    }

    fn parse_tag(&mut self) -> Result<Tag> {
        let ty = self.expect(TokenType::Text, "Expected element name")?;
        let mut attribs = vec![];

        if self.is_next(TokenType::Text) {
            while !(self.is_next(TokenType::LeftBrace) || self.is_next(TokenType::Semicolon)) {
                attribs.push(self.parse_attrib()?);
            }
        }

        let body = self.parse_body()?;
        let tag = Tag {
            ty: ty.lexeme,
            attribs,
            body,
        };

        Ok(tag)
    }

    fn parse_attrib(&mut self) -> Result<(String, Option<String>)> {
        let id = self.expect(TokenType::Text, "Expected attribute name")?;
        
        let value = if self.is_next(TokenType::Equal) {
            self.take()?;
            let value = self.expect(TokenType::String, "Expected string value for attribute")?;
            Some(value.lexeme)
        } else {
            None
        };

        Ok((id.lexeme, value))
    }

    fn parse_body(&mut self) -> Result<Vec<Element>> {
        if self.is_next(TokenType::Semicolon) {
            self.take()?;
            return Ok(vec![]);
        }

        if self.is_next(TokenType::LeftBrace) {
            self.take()?;

            let mut body = vec![];

            while !self.is_next(TokenType::RightBrace) {
                let tag = self.parse_element()?.ok_or_else(|| ParseError::ExpectedTag(self.take().unwrap()))?;
                body.push(tag);
            }

            self.take()?;

            return Ok(body);
        }

        let body = self.parse_element()?.ok_or_else(|| ParseError::ExpectedElement(self.take().unwrap()))?;
        Ok(vec![body])
    }

    fn is_next(&mut self, ty: TokenType) -> bool {
        match self.peek() {
            Ok(Some(t)) if t.ty == ty => true,
            _ => false,
        }
    }

    fn expect(&mut self, expected: TokenType, msg: &str) -> Result<Token> {
        if self.is_next(expected) {
            Ok(self.take()?.unwrap())
        } else {
            Err(ParseError::UnexpectedToken(msg.to_owned(), self.take()?))?
        }
    }

    fn take(&mut self) -> Result<Option<Token>, ScanError> {
        if self.unused.len() > 0 {
            return Ok(self.unused.pop());
        }

        let tok = self.scanner.scan()?;

        if tok.is_none() {
            return Ok(None);
        }

        Ok(Some(tok.unwrap()))
    }

    fn peek(&mut self) -> Result<Option<&Token>> {
        if self.unused.len() == 0 {
            let tok = self.scanner.scan()?;

            if tok.is_none() {
                return Ok(None);
            }

            self.unused.push(tok.unwrap());
        }

        Ok(self.unused.get(self.unused.len() - 1))
    }
}
