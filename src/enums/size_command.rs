use clap::Subcommand;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

use crate::enums::{ DroprError, FileSize, ImageType, SearchResult };
use crate::traits::{ToFileSize, ToFsEntryType, ToImageType, ToSearchResult};
use crate::types::{Cli, Controller};

type Result<T> = std::result::Result<T,DroprError>;

#[derive(Clone,Debug,PartialEq,Subcommand)]
pub enum SizeCommand {
    // Local directory target
    Local,

    // Remote directory target
    Remote
}

impl SizeCommand {

    // returns the upload size of files in a directory, filtering by the search_opt arg if present
    fn local<'a> (&self, path_opt: Option<&PathBuf>, search_opt: Option<&ImageType>) -> Result<u64> {
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
        let mut upload_size = 0_u64;

        for dir_entry in path.read_dir()? {
            let handle = &dir_entry?;

            let search_result = handle
                .to_fs_entry_type()?
                .to_image_type()
                .to_search_result(search_opt);

            let data_to_upload = match search_result {
                SearchResult::Found => handle.metadata()?.size(),
                SearchResult::NoSearchFilter => handle.metadata()?.size(),
                SearchResult::NotFound => 0_u64,
            };

            upload_size += data_to_upload;
        }

        Ok(upload_size)
    }

    async fn remote(&self) -> Result<u64> {
        todo!()
    }

    pub async fn run (&self, args: &Cli, controller: &Controller) -> Result<()> {
        let path_opt = args.path.as_ref();
        let search_opt = args.file_type.as_ref();
        
        let result = match self {
            SizeCommand::Local => self.local(path_opt, search_opt)?,
            SizeCommand::Remote => self.remote().await?
        };

        let result = result as u64;
        
        let display_string = match result.to_file_size()? {
            FileSize::Byte(n) => format!("Total upload size: {n} bytes"),
            FileSize::Kilobte(n) => format!("Total upload size: {n} KB"),
            FileSize::Megabyte(n) => format!("Total upload size: {n} MB"),
            FileSize::GigaByte(n) => format!("Total upload size: {n} GB"),
            FileSize::TeraByte(n) => format!("Total upload size: {n} TB"),
            FileSize::PetaByte(n) => format!("Total upload size: {n} PetaBytes")
        };

        controller.view.header("Results")?
            .println(&display_string)
            .end();
            
        Ok(())
    }
}