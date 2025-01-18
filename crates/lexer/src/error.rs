use {
    miette::{Diagnostic, LabeledSpan, SourceSpan},
    thiserror::Error,
};

/// Error type for the lexer.
#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("Unexpected character: {c}")]
    #[diagnostic(code(lexer::unexpected_char))]
    UnexpectedChar {
        #[source_code]
        src: String,

        c: char,

        #[label("here")]
        at: SourceSpan,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
