use crate::{
    token::{Token, TokenLocation, TokenSpan, TokenType},
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
    apos: usize,

    /// Current line number.
    line: usize,

    /// Relative position, starting from the beginning of the line.
    rpos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            src: input,
            rem: input,
            apos: 0,
            line: 1,
            rpos: 1,
        }
    }

    pub fn tokens(self) -> impl Iterator<Item = Result<Token<'a>>> {
        let eof_token = TokenType::Eof.into();
        self.chain(std::iter::once(Ok(eof_token)))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        // Read until the full lexeme is consumed, then return it wrapped into token.
        loop {
            let c = self.rem.chars().next()?;
            let lexeme = &self.rem[..c.len_utf8()];
            let pos = self.rpos;
            let line = self.line;
            self.rem = &self.rem[c.len_utf8()..];
            self.rpos += c.len_utf8();
            self.apos += c.len_utf8();

            let wrap = |token_type: TokenType| {
                Some(Ok(Token {
                    token_type,
                    lexeme,
                    loc: TokenLocation::new(line, pos),
                    span: TokenSpan::new(self.apos - c.len_utf8(), c.len_utf8()),
                }))
            };

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
                    self.rpos = 1;
                    continue;
                }
                c => {
                    return Some(Err(Error::UnexpectedChar {
                        c,
                        src: self.src.to_string(),
                        at: (self.apos - c.len_utf8(), c.len_utf8()).into(),
                    }))
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        pretty_assertions::{assert_eq, assert_ne},
    };

    fn assert_tokens(input: &str, expected: Vec<Token<'_>>) {
        let lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer.tokens().map(Result::unwrap).collect();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn punctuators() {
        // "(){};,+-*!===<=>=!=<>/."
        assert_tokens("(){};,+-*", vec![
            Token {
                token_type: TokenType::LeftParen,
                lexeme: "(",
                loc: TokenLocation::new(1, 1),
                span: TokenSpan::new(0, 1),
            },
            Token {
                token_type: TokenType::RightParen,
                lexeme: ")",
                loc: TokenLocation::new(1, 2),
                span: TokenSpan::new(1, 1),
            },
            Token {
                token_type: TokenType::LeftBrace,
                lexeme: "{",
                loc: TokenLocation::new(1, 3),
                span: TokenSpan::new(2, 1),
            },
            Token {
                token_type: TokenType::RightBrace,
                lexeme: "}",
                loc: TokenLocation::new(1, 4),
                span: TokenSpan::new(3, 1),
            },
            Token {
                token_type: TokenType::Semicolon,
                lexeme: ";",
                loc: TokenLocation::new(1, 5),
                span: TokenSpan::new(4, 1),
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: ",",
                loc: TokenLocation::new(1, 6),
                span: TokenSpan::new(5, 1),
            },
            Token {
                token_type: TokenType::Plus,
                lexeme: "+",
                loc: TokenLocation::new(1, 7),
                span: TokenSpan::new(6, 1),
            },
            Token {
                token_type: TokenType::Minus,
                lexeme: "-",
                loc: TokenLocation::new(1, 8),
                span: TokenSpan::new(7, 1),
            },
            Token {
                token_type: TokenType::Star,
                lexeme: "*",
                loc: TokenLocation::new(1, 9),
                span: TokenSpan::new(8, 1),
            },
            TokenType::Eof.into(),
        ]);
        assert!(true);
    }
}
