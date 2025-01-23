use crate::{
    token::{Token, TokenKind, TokenSpan},
    Error,
    Result,
};

/// Streaming lexer that produces tokens from the input source.
pub struct Lexer<'a> {
    /// Reference to input source code.
    src: &'a str,

    /// Absolute position starting from the beginning of input code.
    pos: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer instance from the input source code.
    pub fn new(input: &'a str) -> Self {
        Self { src: input, pos: 0 }
    }

    /// Returns an iterator over the tokens in the source code.
    pub fn tokens(self) -> impl Iterator<Item = Result<Token<'a>>> {
        let offset = self.src.len();
        self.chain(std::iter::once(Ok(Token::eof(offset))))
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
            self.pos += c.len_utf8();

            let wrap = |token_type: TokenKind, (start, end): (usize, usize)| {
                let span = TokenSpan::new(start, end);
                Some(Ok(Token {
                    kind: token_type,
                    lexeme: &self.src[span.range()],
                    span,
                }))
            };

            let mut op_with_eq = |op_eq, op| {
                if self.remaining().starts_with('=') {
                    let c = self.remaining().chars().next()?;
                    self.pos += c.len_utf8();
                    wrap(op_eq, (start, self.pos))
                } else {
                    wrap(op, (start, self.pos))
                }
            };

            let is_alphanumeric = |c: char| c.is_ascii_alphanumeric() || c == '_';

            match c {
                '(' | ')' | '{' | '}' | ',' | '.' | '-' | '+' | ';' | '*' => {
                    return wrap(c.into(), (start, self.pos))
                }
                '!' => return op_with_eq(TokenKind::BangEqual, TokenKind::Bang),
                '=' => return op_with_eq(TokenKind::EqualEqual, TokenKind::Equal),
                '>' => return op_with_eq(TokenKind::GreaterEqual, TokenKind::Greater),
                '<' => return op_with_eq(TokenKind::LessEqual, TokenKind::Less),
                '/' => {
                    if self.remaining().starts_with('/') {
                        // Skip the comment until the end of the line.
                        self.pos = self
                            .remaining()
                            .find('\n')
                            .map(|i| self.pos + i)
                            .unwrap_or_else(|| self.src.len());
                        continue;
                    }
                    return wrap(TokenKind::Slash, (start, self.pos));
                }
                '"' => {
                    while let Some(c) = self.remaining().chars().next() {
                        self.pos += c.len_utf8();
                        if c == '"' {
                            return wrap(TokenKind::String, (start, self.pos));
                        }
                    }
                    return Some(Err(Error::UnterminatedString {
                        src: self.src.to_string(),
                        at: (start, (self.pos - start).max(1)).into(),
                    }));
                }
                c if c.is_ascii_digit() => {
                    let consume_digits = |lexer: &mut Lexer<'_>| {
                        while let Some(c) = lexer.remaining().chars().next() {
                            if c.is_digit(10) {
                                lexer.pos += c.len_utf8();
                            } else {
                                break;
                            }
                        }
                    };

                    // Consume whole part of the number.
                    consume_digits(self);

                    // If the next char is a dot and the char after that is digit, consider it as
                    // decimal part, and move cursor to consume it as well.
                    if self.remaining().starts_with('.') {
                        let c = self.remaining().chars().nth(1).unwrap_or('\0');
                        if c.is_digit(10) {
                            self.pos += 1;
                            consume_digits(self);
                        }
                    }

                    return wrap(TokenKind::Number, (start, self.pos));
                }
                c if is_alphanumeric(c) => {
                    while let Some(c) = self.remaining().chars().next() {
                        if is_alphanumeric(c) {
                            self.pos += c.len_utf8();
                        } else {
                            break;
                        }
                    }
                    if let Some(keyword) = TokenKind::from_keyword(&self.src[start..self.pos]) {
                        return wrap(keyword, (start, self.pos));
                    }
                    return wrap(TokenKind::Identifier, (start, self.pos));
                }
                '\n' | '\r' | ' ' | '\t' => continue,
                c => {
                    return Some(Err(Error::UnexpectedChar {
                        c,
                        src: self.src.to_string(),
                        at: (self.pos - c.len_utf8(), c.len_utf8()).into(),
                    }))
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, pretty_assertions::assert_eq};

    fn assert_tokens(input: &str, expected: Vec<Token<'_>>) {
        let lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer.tokens().map(Result::unwrap).collect();
        assert_eq!(tokens, expected);
    }

    fn assert_err(input: &str, expected: Error) {
        let lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer.tokens().collect();
        assert_eq!(tokens, vec![Err(expected), Ok(Token::eof(input.len()))]);
    }

    fn wrap(kind: TokenKind, lexeme: &'_ str, (start, end): (usize, usize)) -> Token<'_> {
        Token {
            kind,
            lexeme,
            span: TokenSpan::new(start, end),
        }
    }

    // See:
    // https://github.com/munificent/craftinginterpreters/blob/master/test/scanning/punctuators.lox
    #[test]
    fn punctuators() {
        let wrap = |token_type: TokenKind, lexeme: &'static str, col: usize| Token {
            kind: token_type,
            lexeme,
            span: TokenSpan::new(col - 1, col - 1 + lexeme.len()),
        };

        assert_tokens("(){};,+-*!===<=>=!=<>/.", vec![
            wrap(TokenKind::LeftParen, "(", 1),
            wrap(TokenKind::RightParen, ")", 2),
            wrap(TokenKind::LeftBrace, "{", 3),
            wrap(TokenKind::RightBrace, "}", 4),
            wrap(TokenKind::Semicolon, ";", 5),
            wrap(TokenKind::Comma, ",", 6),
            wrap(TokenKind::Plus, "+", 7),
            wrap(TokenKind::Minus, "-", 8),
            wrap(TokenKind::Star, "*", 9),
            wrap(TokenKind::BangEqual, "!=", 10),
            wrap(TokenKind::EqualEqual, "==", 12),
            wrap(TokenKind::LessEqual, "<=", 14),
            wrap(TokenKind::GreaterEqual, ">=", 16),
            wrap(TokenKind::BangEqual, "!=", 18),
            wrap(TokenKind::Less, "<", 20),
            wrap(TokenKind::Greater, ">", 21),
            wrap(TokenKind::Slash, "/", 22),
            wrap(TokenKind::Dot, ".", 23),
            Token::eof(23),
        ]);
    }

    // https://github.com/munificent/craftinginterpreters/blob/master/test/scanning/strings.lox
    #[test]
    fn strings() {
        let wrap = |token_type: TokenKind, lexeme: &'static str, (start, end): (usize, usize)| {
            let span = TokenSpan::new(start, end);
            Token {
                kind: token_type,
                lexeme,
                span,
            }
        };
        let mut input = r#"
""
"string"
"#;
        assert_tokens(input, vec![
            wrap(TokenKind::String, "\"\"", (1, 3)),
            wrap(TokenKind::String, "\"string\"", (4, 12)),
            Token::eof(13),
        ]);

        input = r#""unterminated string"#;
        assert_err(input, Error::UnterminatedString {
            src: input.to_string(),
            at: (0, 20).into(),
        });

        input = r#"

""#; // last char is unterminated string
        assert_err(input, Error::UnterminatedString {
            src: input.to_string(),
            at: (2, 1).into(),
        });
    }

    // https://github.com/munificent/craftinginterpreters/blob/master/test/scanning/numbers.lox
    #[test]
    fn numbers() {
        let input = r#"
        123
123.456
.456
123."#;
        assert_tokens(input, vec![
            wrap(TokenKind::Number, "123", (9, 12)),
            wrap(TokenKind::Number, "123.456", (13, 20)),
            wrap(TokenKind::Dot, ".", (21, 22)),
            wrap(TokenKind::Number, "456", (22, 25)),
            wrap(TokenKind::Number, "123", (26, 29)),
            wrap(TokenKind::Dot, ".", (29, 30)),
            Token::eof(30),
        ]);
    }

    // https://github.com/munificent/craftinginterpreters/blob/master/test/scanning/identifiers.lox
    #[test]
    fn identifiers() {
        let input = r#"andy formless fo _ _123 _abc ab123
abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_"#;
        assert_tokens(input, vec![
            wrap(TokenKind::Identifier, "andy", (0, 4)),
            wrap(TokenKind::Identifier, "formless", (5, 13)),
            wrap(TokenKind::Identifier, "fo", (14, 16)),
            wrap(TokenKind::Identifier, "_", (17, 18)),
            wrap(TokenKind::Identifier, "_123", (19, 23)),
            wrap(TokenKind::Identifier, "_abc", (24, 28)),
            wrap(TokenKind::Identifier, "ab123", (29, 34)),
            wrap(
                TokenKind::Identifier,
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_",
                (35, 98),
            ),
            Token::eof(98),
        ]);
    }

    // https://github.com/munificent/craftinginterpreters/blob/master/test/scanning/keywords.lox
    #[test]
    fn keywords() {
        let input = r#"and class else false for fun if nil or return super this true var while"#;
        assert_tokens(input, vec![
            wrap(TokenKind::And, "and", (0, 3)),
            wrap(TokenKind::Class, "class", (4, 9)),
            wrap(TokenKind::Else, "else", (10, 14)),
            wrap(TokenKind::False, "false", (15, 20)),
            wrap(TokenKind::For, "for", (21, 24)),
            wrap(TokenKind::Fun, "fun", (25, 28)),
            wrap(TokenKind::If, "if", (29, 31)),
            wrap(TokenKind::Nil, "nil", (32, 35)),
            wrap(TokenKind::Or, "or", (36, 38)),
            wrap(TokenKind::Return, "return", (39, 45)),
            wrap(TokenKind::Super, "super", (46, 51)),
            wrap(TokenKind::This, "this", (52, 56)),
            wrap(TokenKind::True, "true", (57, 61)),
            wrap(TokenKind::Var, "var", (62, 65)),
            wrap(TokenKind::While, "while", (66, 71)),
            Token::eof(71),
        ]);
    }
}
