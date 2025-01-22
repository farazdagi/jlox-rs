use crate::{
    token::{Token, TokenLocation, TokenSpan, TokenType},
    Error,
    Result,
};

/// Streaming lexer that produces tokens from the input source.
pub struct Lexer<'a> {
    /// Reference to input source code.
    src: &'a str,

    /// Absolute position starting from the beginning of input code.
    pos: usize,

    /// Current row/line number, 1-indexed.
    row: usize,

    /// Current column number, 1-indexed, starting from the beginning of the
    /// current line.
    col: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer instance from the input source code.
    pub fn new(input: &'a str) -> Self {
        Self {
            src: input,
            pos: 0,
            row: 1,
            col: 1,
        }
    }

    /// Returns an iterator over the tokens in the source code.
    pub fn tokens(self) -> impl Iterator<Item = Result<Token<'a>>> {
        let eof_token = TokenType::Eof.into();
        self.chain(std::iter::once(Ok(eof_token)))
    }

    /// Returns the remaining part of the source code.
    fn remaining(&self) -> &str {
        self.src.get(self.pos..).unwrap_or("")
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        // Read until the full lexeme is consumed, then return it wrapped into token.
        loop {
            if self.pos >= self.src.len() {
                return None;
            }

            let c = self.remaining().chars().next()?;
            let start = self.pos;
            let col = self.col;
            let row = self.row;
            self.col += 1;
            self.pos += c.len_utf8();

            let wrap = |token_type: TokenType, span: (usize, usize)| {
                let span = TokenSpan::new(span.0, span.1 - span.0);
                Some(Ok(Token {
                    token_type,
                    lexeme: &self.src[span.offset()..span.offset() + span.len()],
                    loc: TokenLocation::new(row, col),
                    span,
                }))
            };

            enum Pass {
                Slash,
                OpWithEq(TokenType, TokenType),
            }

            let next_pass = match c {
                '(' => return wrap(TokenType::LeftParen, (start, self.pos)),
                ')' => return wrap(TokenType::RightParen, (start, self.pos)),
                '{' => return wrap(TokenType::LeftBrace, (start, self.pos)),
                '}' => return wrap(TokenType::RightBrace, (start, self.pos)),
                ',' => return wrap(TokenType::Comma, (start, self.pos)),
                '.' => return wrap(TokenType::Dot, (start, self.pos)),
                '-' => return wrap(TokenType::Minus, (start, self.pos)),
                '+' => return wrap(TokenType::Plus, (start, self.pos)),
                ';' => return wrap(TokenType::Semicolon, (start, self.pos)),
                '*' => return wrap(TokenType::Star, (start, self.pos)),
                '/' => Pass::Slash,
                '!' => Pass::OpWithEq(TokenType::BangEqual, TokenType::Bang),
                '=' => Pass::OpWithEq(TokenType::EqualEqual, TokenType::Equal),
                '>' => Pass::OpWithEq(TokenType::GreaterEqual, TokenType::Greater),
                '<' => Pass::OpWithEq(TokenType::LessEqual, TokenType::Less),
                c if c.is_whitespace() => continue,
                '\n' | '\r' => {
                    self.row += 1;
                    self.col = 1;
                    continue;
                }
                c => {
                    return Some(Err(Error::UnexpectedChar {
                        c,
                        src: self.src.to_string(),
                        at: (self.pos - c.len_utf8(), c.len_utf8()).into(),
                    }))
                }
            };

            match next_pass {
                Pass::Slash => {
                    if self.remaining().starts_with('/') {
                        // Skip the comment until the end of the line.
                        self.pos = self
                            .remaining()
                            .find('\n')
                            .map(|i| self.pos + i)
                            .unwrap_or_else(|| self.src.len());
                        self.row += 1;
                        self.col = 1;
                    } else {
                        return wrap(TokenType::Slash, (start, self.pos));
                    }
                }
                Pass::OpWithEq(op_eq, op) => {
                    if self.remaining().starts_with('=') {
                        let c = self.remaining().chars().next()?;
                        self.pos += c.len_utf8();
                        self.col += 1;
                        return wrap(op_eq, (start, self.pos));
                    } else {
                        return wrap(op, (start, self.pos));
                    }
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

    // See:
    // https://github.com/munificent/craftinginterpreters/blob/master/test/scanning/punctuators.lox
    #[test]
    fn punctuators() {
        let wrap = |token_type: TokenType, lexeme: &'static str, col: usize| Token {
            token_type,
            lexeme,
            loc: TokenLocation::new(1, col),
            span: TokenSpan::new(col - 1, lexeme.len()),
        };

        assert_tokens("(){};,+-*!===<=>=!=<>/.", vec![
            wrap(TokenType::LeftParen, "(", 1),
            wrap(TokenType::RightParen, ")", 2),
            wrap(TokenType::LeftBrace, "{", 3),
            wrap(TokenType::RightBrace, "}", 4),
            wrap(TokenType::Semicolon, ";", 5),
            wrap(TokenType::Comma, ",", 6),
            wrap(TokenType::Plus, "+", 7),
            wrap(TokenType::Minus, "-", 8),
            wrap(TokenType::Star, "*", 9),
            wrap(TokenType::BangEqual, "!=", 10),
            wrap(TokenType::EqualEqual, "==", 12),
            wrap(TokenType::LessEqual, "<=", 14),
            wrap(TokenType::GreaterEqual, ">=", 16),
            wrap(TokenType::BangEqual, "!=", 18),
            wrap(TokenType::Less, "<", 20),
            wrap(TokenType::Greater, ">", 21),
            wrap(TokenType::Slash, "/", 22),
            wrap(TokenType::Dot, ".", 23),
            TokenType::Eof.into(),
        ]);
        assert!(true);
    }
}
