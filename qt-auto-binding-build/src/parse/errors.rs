use failure::Fail;
use std::{io::Error as IoError, result::Result as StdResult};

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", message)]
    IO { message: String, error: IoError },
    #[fail(display = "Cannot compile source code. Check Rust compilation issues")]
    Source,
    #[fail(display = "Duplicated object `{}`", name)]
    DuplicatedObject { name: String },
}
