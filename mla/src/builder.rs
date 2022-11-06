use std::iter::Iterator;
use std::{iter::Peekable, str::Chars};

use crate::token::{FilePosition, Token, TokenType};

///
/// A stream to which you can return the item back.
///
struct CharStream<'l> {
    cursor: FilePosition,
    text: Chars<'l>,
}

impl<'l> Iterator for CharStream<'l> {
    type Item = (FilePosition, char);

    fn next(&mut self) -> Option<(FilePosition, char)> {
        match self.text.next() {
            Some('\n') => {
                let next_item = (self.cursor.clone(), '\n');
                self.cursor.move_to_newline();
                Some(next_item)
            }
            Some(c) => {
                let next_item = (self.cursor.clone(), c);
                self.cursor.offset += 1;
                Some(next_item)
            }
            None => None,
        }
    }
}

impl<'l> CharStream<'l> {
    ///
    /// Creates a new instance.
    ///
    pub fn new(file_path: String, text: &'l str) -> Self {
        let cursor = FilePosition::new(file_path, 0, 0);
        Self {
            cursor,
            text: text.chars(),
        }
    }
}

///
/// Supports to convert a text into a sequence of tokens.
///
pub struct TokenBuilder<'l> {
    stream: Peekable<CharStream<'l>>,
}

impl<'l> TokenBuilder<'l> {
    ///
    /// Creates a new instance.
    ///
    pub fn new(file_path: String, text: &'l str) -> Self {
        Self {
            stream: CharStream::new(file_path, text).peekable(),
        }
    }

    ///
    /// Reads a numeric.
    ///
    fn read_numeric(&mut self) -> Result<Token, ()> {
        let mut number_builder = String::new();
        let first_pos = self.stream.peek().unwrap().0.clone();

        while let Some((_, c)) = self.stream.next_if(|(_, c)| c.is_ascii_digit()) {
            number_builder.push(c);
        }

        match number_builder.parse::<u32>() {
            Ok(num) => Ok(Token::new(first_pos, TokenType::NaturalNumber(num))),
            Err(_) => Err(()),
        }
    }

    ///
    /// Reads an identifier.
    ///
    fn read_ident(&mut self) -> Result<Token, ()> {
        let mut ident_builder = String::new();
        let first_pos = self.stream.peek().unwrap().0.clone();

        while let Some((_, c)) = self.stream.next_if(|(_, c)| c.is_alphanumeric()) {
            ident_builder.push(c);
        }

        Ok(Token::new(first_pos, TokenType::Ident(ident_builder)))
    }

    ///
    /// Reads a token.
    ///
    fn read_token(&mut self) -> Result<Token, ()> {
        match self.stream.peek() {
            Some((_, ':')) => {
                let pos = self.stream.next().unwrap().0;
                if let Some((_, '=')) = self.stream.next() {
                    Ok(Token::new(pos, TokenType::Assign))
                } else {
                    Err(())
                }
            }
            Some((_, '[')) => {
                let pos = self.stream.next().unwrap().0;

                Ok(Token::new(pos, TokenType::LeftBracket))
            }
            Some((_, ']')) => {
                let pos = self.stream.next().unwrap().0;

                Ok(Token::new(pos, TokenType::RightBracket))
            }
            Some((_, '(')) => {
                let pos = self.stream.next().unwrap().0;

                Ok(Token::new(pos, TokenType::LeftParen))
            }
            Some((_, ')')) => {
                let pos = self.stream.next().unwrap().0;

                Ok(Token::new(pos, TokenType::RightParen))
            }
            Some((_, c)) => {
                if c.is_whitespace() {
                    self.stream.next();
                    self.read_token()
                } else if c.is_ascii_digit() {
                    self.read_numeric()
                } else if c.is_alphabetic() {
                    self.read_ident()
                } else {
                    Err(())
                }
            }
            None => Ok(Token::new(FilePosition::default(), TokenType::EOF)),
        }
    }

    ///
    /// Converts a text into a sequence of tokens.
    ///
    pub fn get_token_stream(&mut self) -> Result<Vec<Token>, ()> {
        let mut tokens = Vec::new();
        loop {
            let token = self.read_token()?;
            if token.token_type == TokenType::EOF {
                break;
            } else {
                tokens.push(token);
            }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn token_builder_1() {
        let mut token_builder = TokenBuilder::new("something.file".to_string(), "id := [x] x");

        assert_eq!(
            token_builder.get_token_stream(),
            Ok(vec![
                Token::new(
                    FilePosition::new("something.file".to_string(), 0, 0),
                    TokenType::Ident("id".to_string())
                ),
                Token::new(
                    FilePosition::new("something.file".to_string(), 0, 3),
                    TokenType::Assign
                ),
                Token::new(
                    FilePosition::new("something.file".to_string(), 0, 6),
                    TokenType::LeftBracket
                ),
                Token::new(
                    FilePosition::new("something.file".to_string(), 0, 7),
                    TokenType::Ident("x".to_string())
                ),
                Token::new(
                    FilePosition::new("something.file".to_string(), 0, 8),
                    TokenType::RightBracket
                ),
                Token::new(
                    FilePosition::new("something.file".to_string(), 0, 10),
                    TokenType::Ident("x".to_string())
                )
            ])
        );
    }

    #[test]
    fn token_builder_2() {
        let mut token_builder =
            TokenBuilder::new("something.file".to_string(), "inc := map\n\t([x] add x 1)");

        assert_eq!(
            token_builder.get_token_stream(),
            Ok(vec![
                Token::new(
                    FilePosition::new("something.file".to_string(), 0, 0),
                    TokenType::Ident("inc".to_string())
                ),
                Token::new(
                    FilePosition::new("something.file".to_string(), 0, 4),
                    TokenType::Assign
                ),
                Token::new(
                    FilePosition::new("something.file".to_string(), 0, 7),
                    TokenType::Ident("map".to_string())
                ),
                Token::new(
                    FilePosition::new("something.file".to_string(), 1, 1),
                    TokenType::LeftParen
                ),
                Token::new(
                    FilePosition::new("something.file".to_string(), 1, 2),
                    TokenType::LeftBracket
                ),
                Token::new(
                    FilePosition::new("something.file".to_string(), 1, 3),
                    TokenType::Ident("x".to_string())
                ),
                Token::new(
                    FilePosition::new("something.file".to_string(), 1, 4),
                    TokenType::RightBracket
                ),
                Token::new(
                    FilePosition::new("something.file".to_string(), 1, 6),
                    TokenType::Ident("add".to_string())
                ),
                Token::new(
                    FilePosition::new("something.file".to_string(), 1, 10),
                    TokenType::Ident("x".to_string())
                ),
                Token::new(
                    FilePosition::new("something.file".to_string(), 1, 12),
                    TokenType::NaturalNumber(1)
                ),
                Token::new(
                    FilePosition::new("something.file".to_string(), 1, 13),
                    TokenType::RightParen
                ),
            ])
        );
    }
}
