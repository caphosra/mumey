use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePosition {
    pub file_path: String,
    pub line: u32,
    pub offset: u32,
}

impl FilePosition {
    pub fn new(file_path: String, line: u32, offset: u32) -> Self {
        Self {
            file_path,
            line,
            offset,
        }
    }

    pub fn move_to_newline(&mut self) {
        self.line += 1;
        self.offset = 0;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Ident(String),
    NaturalNumber(u32),
    Assign,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    EOF,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub position: FilePosition,
    pub token_type: TokenType,
}

impl Token {
    pub fn new(position: FilePosition, token_type: TokenType) -> Self {
        Token {
            position,
            token_type,
        }
    }

    pub fn len(self) -> usize {
        match self.token_type {
            TokenType::Ident(ident) => ident.chars().count(),
            TokenType::NaturalNumber(num) => num.to_string().chars().count(),
            TokenType::Assign => 2,
            TokenType::LeftBracket => 1,
            TokenType::RightBracket => 1,
            TokenType::LeftParen => 1,
            TokenType::RightParen => 1,
            TokenType::EOF => 0,
        }
    }
}

pub struct TokenBuilder<'l> {
    cursor: FilePosition,
    text: Chars<'l>,
    unused: Option<(FilePosition, char)>,
}

impl<'l> TokenBuilder<'l> {
    pub fn new(file_path: String, text: &'l str) -> Self {
        let cursor = FilePosition::new(file_path, 0, 0);
        Self {
            cursor,
            text: text.chars(),
            unused: None,
        }
    }

    fn get_next(&mut self) -> Option<(FilePosition, char)> {
        if let Some(unused) = self.unused.clone() {
            self.unused = None;
            Some(unused)
        } else {
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

    fn read_numeric(&mut self) -> Result<Token, ()> {
        let mut number_builder = String::new();
        let first = self.get_next();

        assert!(first.is_some());

        let (first_pos, c) = first.unwrap();
        number_builder.push(c);

        while let Some((pos, c)) = self.get_next() {
            if c.is_ascii_digit() {
                number_builder.push(c);
            } else {
                self.unused = Some((pos, c));
                break;
            }
        }
        match number_builder.parse::<u32>() {
            Ok(num) => Ok(Token::new(first_pos, TokenType::NaturalNumber(num))),
            Err(_) => Err(()),
        }
    }

    fn read_ident(&mut self) -> Result<Token, ()> {
        let mut ident_builder = String::new();
        let first = self.get_next();

        assert!(first.is_some());

        let (first_pos, c) = first.unwrap();
        ident_builder.push(c);

        while let Some((pos, c)) = self.get_next() {
            if c.is_alphanumeric() {
                ident_builder.push(c);
            } else {
                self.unused = Some((pos, c));
                break;
            }
        }
        Ok(Token::new(first_pos, TokenType::Ident(ident_builder)))
    }

    fn read_next_token(&mut self) -> Result<Token, ()> {
        match self.get_next() {
            Some((pos, ':')) => {
                if let Some((_, '=')) = self.get_next() {
                    Ok(Token::new(pos, TokenType::Assign))
                } else {
                    Err(())
                }
            }
            Some((pos, '[')) => Ok(Token::new(pos, TokenType::LeftBracket)),
            Some((pos, ']')) => Ok(Token::new(pos, TokenType::RightBracket)),
            Some((pos, '(')) => Ok(Token::new(pos, TokenType::LeftParen)),
            Some((pos, ')')) => Ok(Token::new(pos, TokenType::RightParen)),
            Some((pos, c)) => {
                if c.is_whitespace() {
                    self.read_next_token()
                } else if c.is_ascii_digit() {
                    self.unused = Some((pos, c));
                    self.read_numeric()
                } else if c.is_alphabetic() {
                    self.unused = Some((pos, c));
                    self.read_ident()
                } else {
                    Err(())
                }
            }
            None => Ok(Token::new(self.cursor.clone(), TokenType::EOF)),
        }
    }

    pub fn get_token_stream(&mut self) -> Result<Vec<Token>, ()> {
        let mut tokens = Vec::new();
        loop {
            let token = self.read_next_token()?;
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
