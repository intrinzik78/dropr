use std::{path::PathBuf,fs::ReadDir};

use clap::Subcommand;

use crate::enums::Error;
use crate::types::View;


type Result<T> = std::result::Result<T,Error>;

#[derive(Clone,Debug,PartialEq,Subcommand)]
pub enum CountCommand {
    /// Local target
    Local,

    /// Dropbox target
    Remote
}

impl CountCommand {
    fn count_local(&self, path_opt: &Option<PathBuf>) -> Result<u32> {
        if let Some(path) = path_opt {
            todo!()
            // let accepted_file_extensions: [&'static str;8] = ["";8];
            
            // let d = ReadDir::filter_map(self, |s|)
        } else {
            todo!()
        }
    }

    async fn count_remote(&self, _path: &PathBuf) -> Result<u32> {
        todo!();
    }

    pub async fn run(&self, path_opt: &Option<PathBuf>, _view: &View) -> Result<u32> {
        let count = match self {
            CountCommand::Local => self.count_local(path_opt)?,
            CountCommand::Remote => {
                if let Some(path) = path_opt {
                    self.count_remote(path).await?
                } else {
                    return Err(Error::NoRemotePathSpecified);
                }
            }
        };

        Ok(count)
    }
}