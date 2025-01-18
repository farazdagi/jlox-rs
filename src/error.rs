use {miette::Diagnostic, thiserror::Error};

/// Main error type for the Lox interpreter.
#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(jlox::io_error))]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic(transparent)]
    LexerError(#[from] lexer::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
