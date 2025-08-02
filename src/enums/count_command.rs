use clap::Subcommand;
use std::path::PathBuf;

use crate::enums::{ Error, ImageType, SearchResult };
use crate::types::View;
use crate::traits::{ ToFsEntryType, ToImageType, ToSearchResult };

type Result<T> = std::result::Result<T,Error>;

#[derive(Clone,Debug,PartialEq,Subcommand)]
pub enum CountCommand {
    /// Local target
    Local,

    /// Dropbox target
    Remote
}

impl CountCommand {

    // counts the number of files in a source directory, filtering by the search_opt arg if present
    fn count_local(&self, path_opt: Option<&PathBuf>, search_opt: Option<&ImageType>) -> Result<u32> {
        // extract path argument or operate on current directory
        let path = match path_opt {
            Some(p_buf) => p_buf,
            None => &std::env::current_dir().map_err(|_e| Error::GetCurrentDirectory)?
        };

        // verify path is a directory
        if !path.is_dir() {   
            return Err(Error::PathNotDirectory)
        }

        // iterate over target directory entries
        let mut counter = 0_u32;

        for dir_entry in path.read_dir()? {
            let handle = &dir_entry?;
            let search_result = handle
                .to_fs_entry_type()?
                .to_image_type()
                .to_search_result(search_opt);

            // filter results
            match search_result {
                SearchResult::Found => counter += 1,
                SearchResult::NoSearchFilter => counter +=1,
                SearchResult::NotFound => {}
            };
        }

        Ok(counter)
    }

    // counts the number of files in a target directory
    async fn count_remote(&self, _path_opt: Option<&PathBuf>) -> Result<u32> {
        todo!();
    }

    // base run count command
    pub async fn run(&self, path_opt: Option<&PathBuf>, search_opt: Option<&ImageType>, view: &View) -> Result<()> {
        let count = match self {
            CountCommand::Local => self.count_local(path_opt, search_opt)?,
            CountCommand::Remote => {
                if path_opt.is_some() {
                    self.count_remote(path_opt).await?
                } else {
                    return Err(Error::NoRemotePathSpecified);
                }
            }
        };

        // display results
        view.header("Results")?
            .println(&format!("{count} files found."))
            .end();
        
        Ok(())
    }

}