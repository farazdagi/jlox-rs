use core::fmt;

/// Types of tokens that the lexer can produce.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // Special tokens.
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::LeftBrace => "{",
            Self::RightBrace => "}",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::Minus => "-",
            Self::Plus => "+",
            Self::Semicolon => ";",
            Self::Slash => "/",
            Self::Star => "*",
            Self::Eof => "<EOF>",
        };
        write!(f, "{out}")
    }
}

/// Token location is a pair of indices representing row and column of the
/// token in the source code.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TokenLocation(usize, usize);

impl TokenLocation {
    pub fn new(row: usize, col: usize) -> Self {
        Self(row, col)
    }

    pub fn row(&self) -> usize {
        self.0
    }

    pub fn col(&self) -> usize {
        self.1
    }
}

impl fmt::Display for TokenLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}:{})", self.0, self.1)
    }
}

/// Token span is a pair of values: an offset from the beginning of the source
/// code and the length of the token.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TokenSpan(usize, usize);

impl TokenSpan {
    pub fn new(offset: usize, len: usize) -> Self {
        Self(offset, len)
    }

    pub fn from_single_char(offset: usize) -> Self {
        Self(offset, 1)
    }

    pub fn offset(&self) -> usize {
        self.0
    }

    pub fn len(&self) -> usize {
        self.1
    }
}

impl fmt::Display for TokenSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}:{})", self.0, self.0 + self.1)
    }
}

/// Token is a lexeme wrapped up with some extra information (useful for
/// successive parsing).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub loc: TokenLocation,
    pub span: TokenSpan,
}

impl<'a> From<TokenType> for Token<'a> {
    fn from(token_type: TokenType) -> Self {
        match token_type {
            _ => Self {
                token_type,
                lexeme: "",
                loc: TokenLocation::default(),
                span: TokenSpan::default(),
            },
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{type: {}, lexeme: {}({}), at {}}}",
            self.token_type, self.lexeme, self.span, self.loc
        )
    }
}
