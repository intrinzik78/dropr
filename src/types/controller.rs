use crate::enums::{
    Error,
    PrimaryCommand
};

use crate::types::{
    Cli,
    DatabaseConnection,
    View
};

type Result<T> = std::result::Result<T,Error>;

#[derive(Debug)]
pub struct Controller {
    pub model: DatabaseConnection,
    pub view: View
}

impl Controller {
    // Model,View,Controller architecture
    pub async fn new() -> Result<Controller> {
        let view = View::new()?;
        let model = DatabaseConnection::new().await?;
        let controller = Controller {
            model,
            view
        };

        Ok(controller)
    }

    pub async fn run(&self, args: &Cli) -> Result<()> {

        match &args.command {
            PrimaryCommand::Count(command) => command.run(&args.path, &self.view).await?,
            _ => return Err(Error::InvalidCommandLineArgument)
        };

        Ok(())
    }
}