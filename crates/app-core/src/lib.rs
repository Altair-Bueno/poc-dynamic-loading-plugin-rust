use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct Context {
    pub invocations: usize,
}

pub trait Plugin {
    fn hook(&mut self, context: &mut Context) -> Result<(), Error>;
}
