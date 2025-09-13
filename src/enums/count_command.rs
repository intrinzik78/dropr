use clap::Subcommand;
use std::{path::PathBuf};

use crate::enums::{ DroprError, ImageType, SearchResult };
use crate::types::{Cli, Controller};
use crate::traits::{ ToFsEntryType, ToImageType, ToSearchResult };

type Result<T> = std::result::Result<T,DroprError>;

#[derive(Clone,Debug,PartialEq,Subcommand)]
pub enum CountCommand {
    /// Local folder target
    Local,

    /// Remote bucket target
    Remote
}

impl CountCommand {

    // counts the number of files in a source directory, filtering by the search_opt arg if present
    fn count_local(&self, path_opt: Option<&PathBuf>, file_type_opt: Option<&ImageType>) -> Result<u32> {
        // extract path argument or operate on current directory
        let path = match path_opt {
            Some(p) => p,
            None => &Controller::current_directory()?
        };

        // verify path is a directory
        if !path.is_dir() {   
            return Err(DroprError::PathNotDirectory)
        }

        // iterate over target directory entries
        let mut counter = 0_u32;

        for dir_entry in path.read_dir()? {
            let handle = &dir_entry?;
            let search_result = handle
                .to_fs_entry_type()?
                .to_image_type()
                .to_search_result(file_type_opt);

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
        Ok(10)
    }

    // base run count command
    pub async fn run(&self, args: &Cli, controller: &mut Controller) -> Result<()> {
        let path_opt = args.path.as_ref();
        let file_type_opt = args.file_type.as_ref();
        
        let count = match self {
            CountCommand::Local => self.count_local(path_opt, file_type_opt)?,
            CountCommand::Remote => {
                controller.login().await?;
                self.count_remote(path_opt).await?
            }
        };

        // display results
        controller.view.header("Results")?
            .println(&format!("{count} files found."))
            .end();
        
        Ok(())
    }

}