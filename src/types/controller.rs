use crate::enums::Error;
use crate::types::{ DatabaseConnection, View };

type Result<T> = std::result::Result<T,Error>;

#[derive(Debug)]
pub struct Controller {
    pub database: DatabaseConnection,
    pub view: View
}

impl Controller {
    pub async fn new() -> Result<Controller> {
        let view = View::new()?;
        let database = DatabaseConnection::new().await?;
        let controller = Controller {
            database,
            view
        };

        Ok(controller)
    }

    pub async fn run(&self) -> Result<()> {
        Ok(())
    }
}