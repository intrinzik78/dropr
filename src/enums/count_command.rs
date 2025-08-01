use clap::Subcommand;
use std::path::PathBuf;

use crate::enums::{Error, ImageType};
use crate::types::View;
use crate::traits::{ ToFsEntryType, ToImageType };

type Result<T> = std::result::Result<T,Error>;

#[derive(Clone,Debug,PartialEq,Subcommand)]
pub enum CountCommand {
    /// Local target
    Local,

    /// Dropbox target
    Remote
}

#[derive(Debug,PartialEq)]
enum SearchResult {
    Found,
    NotFound
}

impl CountCommand {

    fn filter(image_type_opt: Option<ImageType>, find_type_opt: &Option<ImageType>) -> SearchResult {
        // early return if no filter
        if find_type_opt.is_none() || image_type_opt.is_none(){
            return SearchResult::Found;
        }

        if image_type_opt == *find_type_opt {
            return SearchResult::Found
        } else {
            return SearchResult::NotFound;
        }
    }

    fn count_local(&self, path_opt: &Option<PathBuf>, find_type_opt: &Option<ImageType>) -> Result<u32> {
        // extract path argument or operate on current directory
        let path = match path_opt {
            Some(p_buf) => p_buf,
            None => &std::env::current_dir().map_err(|_e| Error::GetCurrentDirectory)?
        };

        // verify path is a directory
        if !path.is_dir() {   
            return Err(Error::PathNotDirectory)
        }

        // iterate over directory entries
        let mut counter = 0_u32;

        for entry_result in path.read_dir()? {
            let image_type_opt = entry_result?
                .to_fs_entry_type()?
                .to_image_type();

            // filter results
            if CountCommand::filter(image_type_opt,find_type_opt) == SearchResult::Found {
                counter += 1;
            }
        }

        Ok(counter)
    }

    async fn count_remote(&self, _path: &PathBuf) -> Result<u32> {
        todo!();
    }

    pub async fn run(&self, path_opt: &Option<PathBuf>, search_opt: &Option<ImageType>, view: &View) -> Result<()> {
        let count = match self {
            CountCommand::Local => self.count_local(path_opt, search_opt)?,
            CountCommand::Remote => {
                if let Some(path) = path_opt {
                    self.count_remote(path).await?
                } else {
                    return Err(Error::NoRemotePathSpecified);
                }
            }
        };

        // display results
        view.header("Results")?
            .println(&format!("{} files found.", count))
            .end();
        
        Ok(())
    }
}