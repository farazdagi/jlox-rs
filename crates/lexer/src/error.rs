use {
    miette::{Diagnostic, SourceSpan},
    thiserror::Error,
};

/// Error type for the lexer.
#[derive(Error, Diagnostic, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Unexpected character: {c}")]
    #[diagnostic(code(lexer::unexpected_char))]
    UnexpectedChar {
        #[source_code]
        src: String,
        #[label("here")]
        at: SourceSpan,
        c: char,
    },

    #[error("Source code ended unexpectedly")]
    #[diagnostic(code(lexer::unexpected_eof))]
    UnexpectedEof {
        #[source_code]
        src: String,
        #[label("here")]
        at: SourceSpan,
    },

    #[error("Unterminated string")]
    #[diagnostic(code(lexer::unterminated_string))]
    UnterminatedString {
        #[source_code]
        src: String,
        #[label("here")]
        at: SourceSpan,
    },

    #[error("Unterminated block comment")]
    #[diagnostic(code(lexer::unterminated_block_comment))]
    UnterminatedBlockComment {
        #[source_code]
        src: String,
        #[label("here")]
        at: SourceSpan,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
