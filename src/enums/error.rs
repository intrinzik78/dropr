use std::fmt;
use derive_more::From;
use sqlx;

#[derive(Debug, From)]
pub enum Error {
    // derived error generation from dependencies that implement Error
    #[from]
    Sqlx(sqlx::Error),
    
    // internal errors
    InvalidCommandLineArgument,
    NoRemotePathSpecified,
    RemotePathDoesNotExist,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidCommandLineArgument => write!(f, "Invalid command line argument. Use --help to see available options and commands."),
            Error::NoRemotePathSpecified => write!(f, "Path required to count files in a remote photo bucket."),
            Error::RemotePathDoesNotExist => write!(f, "Bucket does not exist. Use --list-buckets to see which buckets are online."),
            _ => write!(f, "{self:?}")
        }
    }
}
