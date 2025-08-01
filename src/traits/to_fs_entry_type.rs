use std::fs::DirEntry;

use crate::enums::{ Error, FsEntryType };

type Result<T> = std::result::Result<T,Error>;

pub trait ToFsEntryType {
    fn to_fs_entry_type(self) -> Result<FsEntryType>;
}

impl ToFsEntryType for DirEntry {
    fn to_fs_entry_type(self) -> Result<FsEntryType> {
        if self.file_type()?.is_dir() {
            return Ok(FsEntryType::Directory);
        }

        if self.file_type()?.is_file() {
            let filename = match self.file_name().into_string() {
                Ok(converted_string) => converted_string,
                Err(os_string) => {
                    let byte_string = os_string.as_encoded_bytes().to_vec();
                    String::from_utf8(byte_string)?
                }
            };

            let filename = filename.to_ascii_lowercase();

            return Ok(FsEntryType::File(filename));
        }

        if self.file_type()?.is_symlink() {
            return Ok(FsEntryType::SymLink);
        }

        return Err(Error::NoValidFsEntryType);
    }
}