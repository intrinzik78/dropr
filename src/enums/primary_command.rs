use clap::Subcommand;

use crate::enums::{ CountCommand,SizeCommand };

#[derive(Clone,Debug,Subcommand,PartialEq)]
pub enum PrimaryCommand {
    
    #[command(subcommand)]
    /// count the files in a local or remote folder
    Count(CountCommand),

    /// create a remote dropbox photo bucket with associated database entries
    Create,

    /// delete a photo bucket and associated database entries
    Delete,

    /// list photo buckets
    ListBuckets,

    /// rename a photo bucket
    Rename,

    /// upload size of a target path
    #[command(subcommand)]
    Size(SizeCommand)
    
}