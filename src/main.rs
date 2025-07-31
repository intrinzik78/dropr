use clap::Parser;

pub mod types;
pub mod enums;
pub mod traits;

use enums::Error;
use types::{ Cli, Controller };

type Result<T> = std::result::Result<T,Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let with_cli_arguments = Cli::parse();
    let controller = Controller::new().await?;
    
    controller
        .run(&with_cli_arguments)
        .await?;

    Ok(())
}
