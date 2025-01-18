use std::env;

mod error;
mod lox;

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum Error {
    #[error("Invalid argumets: {0}")]
    #[diagnostic(code(jlox::invalid_args))]
    InvalidArgs(String),
}

fn main() -> miette::Result<()> {
    let lox = lox::Lox::new();

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => lox.run_prompt().map_err(Into::into),
        2 => lox.run_file(&args[1]).map_err(Into::into),
        _ => Err(Error::InvalidArgs(format!("Usage: {} [script]", args[0])).into()),
    }
}
