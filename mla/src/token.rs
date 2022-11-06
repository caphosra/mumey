///
/// Holds a location on a file.
///
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FilePosition {
    pub file_path: String,
    pub line: u32,
    pub offset: u32,
}

impl FilePosition {
    ///
    /// Creates a new instance.
    ///
    pub fn new(file_path: String, line: u32, offset: u32) -> Self {
        Self {
            file_path,
            line,
            offset,
        }
    }

    ///
    /// Seeks to the next line. This operation also resets the offset.
    ///
    pub fn move_to_newline(&mut self) {
        self.line += 1;
        self.offset = 0;
    }
}

///
/// Types of tokens available.
///
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

///
/// Represents a token. Consists of the position and the type of token.
///
#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub position: FilePosition,
    pub token_type: TokenType,
}

impl Token {
    ///
    /// Creates a new instance.
    ///
    pub fn new(position: FilePosition, token_type: TokenType) -> Self {
        Token {
            position,
            token_type,
        }
    }

    ///
    /// Returns a length of it.
    ///
    pub fn len(&self) -> usize {
        match &self.token_type {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn token_1() {
        let token = Token::new(
            FilePosition::new("something.file".to_string(), 5, 2),
            TokenType::Ident("everything".to_string()),
        );

        assert_eq!(token.len(), 10);
    }
}
