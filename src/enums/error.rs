use std::{fmt, io::Error as IOError, string::FromUtf8Error};
use derive_more::From;
use sqlx;

#[derive(Debug, From)]
pub enum Error {
    // derived error generation from dependencies that implement Error
    #[from]
    Sqlx(sqlx::Error),

    #[from]
    Utf8Conversion(FromUtf8Error),

    #[from]
    IoError(IOError),
    
    // internally mapped errors
    ImageTypeNotRecognized,
    FailedToFlushStdOut,
    GetCurrentDirectory,
    InvalidCommandLineArgument,
    NoRemotePathSpecified,
    NoValidFsEntryType,
    PathNotDirectory,
    RemotePathDoesNotExist,
    SystemLocked
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FailedToFlushStdOut => write!(f,"Error printing to display."),
            Error::Utf8Conversion(_) => write!(f,"System error reading filenames in target directory."),
            Error::ImageTypeNotRecognized => write!(f,"Image type not recognized. Use --help to see supported file types."),
            Error::GetCurrentDirectory => write!(f, "Failed to get current directory. Check permissions or use the explicit --path argument."),
            Error::InvalidCommandLineArgument => write!(f, "Invalid command line argument. Use --help to see available options and commands."),
            Error::NoRemotePathSpecified => write!(f, "Path required to count files in a remote photo bucket."),
            Error::NoValidFsEntryType => write!(f, "Could not read file types in file system."),
            Error::PathNotDirectory => write!(f, "Path is not a directory. Check the path and try again."),
            Error::RemotePathDoesNotExist => write!(f, "Bucket does not exist. Use --list-buckets to see which buckets are online."),
            Error::SystemLocked => write!(f, "Unable to unlock system. Retry or contact developer for a system key."),
            _ => write!(f, "{self:?}")
        }
    }
}
