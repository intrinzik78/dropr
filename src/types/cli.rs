use clap::{ Parser };
use std::path::PathBuf;

use crate::enums::PrimaryCommand;


#[derive(Debug,Parser)]
#[command(name = "Dropr", about = "Create and manage dropbox photo buckets and file links for public download")]
pub struct Cli {
    
    /// Primary command to run
    #[command(subcommand)]
    pub command: PrimaryCommand,

    /// Path to the target directory
    #[arg(long,short)]
    pub path: Option<PathBuf>

}