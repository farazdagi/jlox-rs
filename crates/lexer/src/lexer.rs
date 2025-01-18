use crate::{
    token::{Token, TokenType},
    Error,
    Result,
};

/// Streaming lexer that produces tokens from the input source.
pub struct Lexer<'a> {
    /// Whole input source.
    src: &'a str,

    /// Remaining input source.
    rem: &'a str,

    /// Absolute position starting from the beginning of input code.
    abs_pos: usize,

    /// Relative position, starting from the beginning of the line.
    rel_pos: usize,

    /// Current line number.
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            src: input,
            rem: input,
            abs_pos: 0,
            rel_pos: 0,
            line: 0,
        }
    }

    pub fn tokens(self) -> impl Iterator<Item = Result<Token<'a>>> {
        self.chain(std::iter::once(Ok(Token {
            token_type: TokenType::Eof,
            lexeme: "",
            line: 0,
            offset: 0,
        })))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.rem.chars().next()?;
        let lexeme = &self.rem[..c.len_utf8()];
        let pos = self.rel_pos;
        let line = self.line;
        self.rem = &self.rem[c.len_utf8()..];
        self.rel_pos += c.len_utf8();
        self.abs_pos += c.len_utf8();

        let wrap = |token_type: TokenType| {
            Some(Ok(Token {
                token_type,
                lexeme,
                line,
                offset: pos,
            }))
        };

        // Read until the full lexeme is consumed, then return it wrapped into token.
        loop {
            match c {
                '(' => return wrap(TokenType::LeftParen),
                ')' => return wrap(TokenType::RightParen),
                '{' => return wrap(TokenType::LeftBrace),
                '}' => return wrap(TokenType::RightBrace),
                ',' => return wrap(TokenType::Comma),
                '.' => return wrap(TokenType::Dot),
                '-' => return wrap(TokenType::Minus),
                '+' => return wrap(TokenType::Plus),
                ';' => return wrap(TokenType::Semicolon),
                '*' => return wrap(TokenType::Star),

                // TODO: Slash needs different handling (it is division and comment)
                '/' => return wrap(TokenType::Slash),
                c if c.is_whitespace() => continue,
                '\n' | '\r' => {
                    self.line += 1;
                    self.rel_pos = 0;
                    continue;
                }
                c => {
                    return Some(Err(Error::UnexpectedChar {
                        c,
                        src: self.src.to_string(),
                        at: (self.abs_pos - c.len_utf8(), c.len_utf8()).into(),
                    }))
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert!(true);
    }
}
