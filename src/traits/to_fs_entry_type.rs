use std::fs::DirEntry;

use crate::enums::{ Error, FsEntryType };

type Result<T> = std::result::Result<T,Error>;

pub trait ToFsEntryType {
    fn to_fs_entry_type(self) -> Result<FsEntryType>;
}

impl ToFsEntryType for &DirEntry {
    fn to_fs_entry_type(self) -> Result<FsEntryType> {

        // directory
        if self.file_type()?.is_dir() {
            return Ok(FsEntryType::Directory);
        }

        // file-entry
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

        // sym-link
        if self.file_type()?.is_symlink() {
            return Ok(FsEntryType::SymLink);
        }

        Err(Error::NoValidFsEntryType)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;

    #[test]
    // this test builds a temporary file structure (using tempfile crate) then iterates over the files to test the ToFsEntryType trait
    fn to_fs_entry_type() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // create a subdirectory
        let sub_dir_path = dir_path.join("subdir");
        fs::create_dir(&sub_dir_path).unwrap();

        // create a file
        let file_path = dir_path.join("file.txt");
        File::create(&file_path).unwrap();

        // create a symlink (if supported)
        #[cfg(unix)]
        std::os::unix::fs::symlink(&file_path, dir_path.join("symlink")).unwrap();


        for entry in fs::read_dir(dir_path).expect("failed to read directory") {
            // consume DirEntry immediately and extract test data
            let handle = &entry.expect("failed to get file entry handle");
            let filename = handle.file_name();
            let trait_test = handle.to_fs_entry_type().expect("failed to get create trait: FsEntryType");
            
            match filename.into_string().unwrap().as_str() {
                "subdir" => assert_eq!(trait_test,FsEntryType::Directory),
                "file.txt" => assert_eq!(trait_test,FsEntryType::File("file.txt".to_string())),
                #[cfg(unix)]
                "symlink" => assert_eq!(trait_test,FsEntryType::SymLink),
                _ => panic!("panic, unhandled data found in test")
            };
        }
    }
}