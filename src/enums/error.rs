use std::{fmt, io::Error as IOError, string::FromUtf8Error};
use derive_more::From;

use idropr_sdk_rust::{enums::SDKError};

#[derive(Debug, From)]
pub enum DroprError {
    #[from]
    Utf8Conversion(FromUtf8Error),

    #[from]
    IoError(IOError),

    #[from]
    SDKError(SDKError),
    
    // internally mapped errors
    ApiError(String),
    ImageTypeNotRecognized,
    FailedToFlushStdOut,
    GetCurrentDirectory,
    InvalidCommandLineArgument,
    FailedLogin,
    NoRemotePathSpecified,
    NoValidPortProvided,
    NoValidFsEntryType,
    PathNotDirectory,
    RemotePathDoesNotExist,
    StdReadPasswordError,
    StdReadLineError,
}

impl std::error::Error for DroprError {}

impl fmt::Display for DroprError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DroprError::ApiError(m) => write!(f,"{m}"),
            DroprError::FailedToFlushStdOut => write!(f,"Error printing to display."),
            DroprError::Utf8Conversion(_) => write!(f,"System error reading filenames in target directory."),
            DroprError::ImageTypeNotRecognized => write!(f,"Image type not recognized. Use --help to see supported file types."),
            DroprError::GetCurrentDirectory => write!(f, "Failed to get current directory. Check permissions or use the explicit --path argument."),
            DroprError::FailedLogin => write!(f,"Could not login. Check your username / password and try again."),
            DroprError::InvalidCommandLineArgument => write!(f, "Invalid command line argument. Use --help to see available options and commands."),
            DroprError::NoRemotePathSpecified => write!(f, "Path required to count files in a remote photo bucket."),
            DroprError::NoValidFsEntryType => write!(f, "Could not read file types in file system."),
            DroprError::PathNotDirectory => write!(f, "Path is not a directory. Check the path and try again."),
            DroprError::RemotePathDoesNotExist => write!(f, "Bucket does not exist. Use --list-buckets to see which buckets are online."),
            _ => write!(f, "{self:?}")
        }
    }
}
