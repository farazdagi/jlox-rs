use {
    crate::error::{Error, Result},
    lexer::Lexer,
    std::io::Write,
};

/// Lox language interpreter.
pub struct Lox {}

impl Lox {
    /// Create a new Lox interpreter.
    pub fn new() -> Self {
        Self {}
    }

    /// Run a Lox script from a file.
    pub fn run_file(&self, filename: &str) -> Result<()> {
        let file_contents = std::fs::read_to_string(filename).unwrap_or_else(|e| {
            self.error(e);
            String::new()
        });
        self.run(&file_contents)
    }

    /// Run a Lox REPL.
    pub fn run_prompt(&self) -> Result<()> {
        let input = std::io::stdin();
        let mut stdout = std::io::stdout().lock();
        loop {
            stdout.write_all(b"> ")?;
            stdout.flush()?;
            let mut buffer = String::new();
            input.read_line(&mut buffer).unwrap();
            if buffer.is_empty() {
                break;
            }
            self.run(&buffer)?
        }
        Ok(())
    }

    /// Run a Lox source code.
    pub fn run(&self, source: &str) -> Result<()> {
        let lexer = Lexer::new(source);
        for token in lexer.tokens() {
            match token {
                Ok(token) => {
                    println!("{:?}", token);
                }
                // Print error and continue parsing.
                // This allows to report multiple errors in a single run.
                Err(e) => self.error(e),
            }
        }
        Ok(())
    }

    fn error<E: Into<Error>>(&self, e: E) {
        eprintln!("{:?}", miette::Report::new(e.into()));
    }
}
