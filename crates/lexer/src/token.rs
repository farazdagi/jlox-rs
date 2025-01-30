use core::fmt;

/// Types of tokens that the lexer can produce.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
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

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    String,
    Identifier,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Special tokens.
    Eof,
}

/// Token kinds that are reserved keywords of the language.
pub const KEYWORDS: &[(&str, TokenKind)] = &[
    ("and", TokenKind::And),
    ("class", TokenKind::Class),
    ("else", TokenKind::Else),
    ("false", TokenKind::False),
    ("for", TokenKind::For),
    ("fun", TokenKind::Fun),
    ("if", TokenKind::If),
    ("nil", TokenKind::Nil),
    ("or", TokenKind::Or),
    ("print", TokenKind::Print),
    ("return", TokenKind::Return),
    ("super", TokenKind::Super),
    ("this", TokenKind::This),
    ("true", TokenKind::True),
    ("var", TokenKind::Var),
    ("while", TokenKind::While),
];

impl TokenKind {
    /// Returns the token kind for a given keyword if it is a reserved keyword.
    pub fn from_keyword(lexeme: &str) -> Option<Self> {
        KEYWORDS.iter().find_map(|(keyword, kind)| {
            if lexeme == *keyword {
                Some(*kind)
            } else {
                None
            }
        })
    }
}

impl From<char> for TokenKind {
    fn from(c: char) -> Self {
        match c {
            '(' => Self::LeftParen,
            ')' => Self::RightParen,
            '{' => Self::LeftBrace,
            '}' => Self::RightBrace,
            ',' => Self::Comma,
            '.' => Self::Dot,
            '-' => Self::Minus,
            '+' => Self::Plus,
            ';' => Self::Semicolon,
            '/' => Self::Slash,
            '*' => Self::Star,
            '!' => Self::Bang,
            '=' => Self::Equal,
            '>' => Self::Greater,
            '<' => Self::Less,
            _ => panic!("Invalid character: {}", c),
        }
    }
}

impl fmt::Display for TokenKind {
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
            Self::Bang => "!",
            Self::BangEqual => "!=",
            Self::Equal => "=",
            Self::EqualEqual => "==",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::String => "string",
            Self::Identifier => "identifier",
            Self::Number => "number",
            Self::And => "and",
            Self::Class => "class",
            Self::Else => "else",
            Self::False => "false",
            Self::Fun => "fun",
            Self::For => "for",
            Self::If => "if",
            Self::Nil => "nil",
            Self::Or => "or",
            Self::Print => "print",
            Self::Return => "return",
            Self::Super => "super",
            Self::This => "this",
            Self::True => "true",
            Self::Var => "var",
            Self::While => "while",
            Self::Eof => "<EOF>",
        };
        write!(f, "{out}")
    }
}

/// Represents a span of bytes in the source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenSpan(usize, usize);

impl TokenSpan {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(end >= start);
        Self(start, end)
    }

    pub fn start(&self) -> usize {
        self.0
    }

    pub fn end(&self) -> usize {
        self.1
    }

    pub fn length(&self) -> usize {
        assert!(self.1 >= self.0);
        self.1 - self.0
    }

    pub fn range(&self) -> std::ops::Range<usize> {
        self.0..self.1
    }
}

impl fmt::Display for TokenSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}:{})", self.0, self.0 + self.1)
    }
}

/// Token is a lexeme wrapped up with some extra information (useful for
/// successive parsing).
#[derive(Debug, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str,
    pub span: TokenSpan,
}

impl<'a> Token<'a> {
    pub fn eof(offset: usize) -> Self {
        Self {
            kind: TokenKind::Eof,
            lexeme: "<eof>",
            span: TokenSpan::new(offset, offset),
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{type: {}, lexeme: {}({})}}",
            self.kind, self.lexeme, self.span,
        )
    }
}
