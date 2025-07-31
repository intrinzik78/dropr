pub mod types;
pub mod enums;
pub mod traits;

use enums::Error;
use types::Controller;

type Result<T> = std::result::Result<T,Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let controller = Controller::new().await?;
    
    controller
        .run()
        .await?;

    Ok(())
}
