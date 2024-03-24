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

pub trait Plugin {
    fn init(&mut self) -> Result<(), Error>;
    fn update(&mut self) -> Result<(), Error>;
    fn deinit(&mut self) -> Result<(), Error>;
}
