use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
pub enum Error {
    UnknownDocument,
    DocumentAlreadyExists,
}

impl From<Error> for lsp_server::Response {
    fn from(_e: Error) -> Self {
        todo!();
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::UnknownDocument => "unknown document uri",
            Error::DocumentAlreadyExists => "document already exists",
        })
    }
}

impl std::error::Error for Error {}
