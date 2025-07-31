use clap::{ Parser };
use std::path::PathBuf;

use crate::enums::{ImageType, PrimaryCommand};

#[derive(Debug,Parser)]
#[command(name = "Dropr", about = "Create and manage dropbox photo buckets and file links for public download")]
pub struct Cli {
    
    /// Primary command to run
    #[command(subcommand)]
    pub command: PrimaryCommand,

    /// Specify an image type
    #[arg(long,short)]
    pub file_type: Option<ImageType>,

    /// Path to the target directory
    #[arg(long,short)]
    pub path: Option<PathBuf>
}