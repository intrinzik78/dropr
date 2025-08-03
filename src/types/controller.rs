use crate::enums::{
    Error,
    PrimaryCommand,
    SystemStatus
};

use crate::types::{
    Cli,
    DatabaseConnection,
    Dropbox,
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
        let view = &self.view;
        let path_opt = args.path.as_ref();
        let find_type_opt = args.find_type.as_ref();
        
        let master_password = view.prompt_private("Enter password: ")?;

        match SystemStatus::unlock(&master_password).await? {
            SystemStatus::Locked => return Err(Error::SystemLocked),
            SystemStatus::Unlocked => {}
        }
        
        match &args.command {
            PrimaryCommand::Count(command) => command.run(path_opt, find_type_opt, view).await?,
            PrimaryCommand::Size(command) => command.run(path_opt, find_type_opt, view).await?,
            _ => return Err(Error::InvalidCommandLineArgument)
        };

        Ok(())
    }
}