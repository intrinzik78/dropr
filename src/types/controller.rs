use crate::enums::{
    Error,
    PrimaryCommand,
    SessionToken
};

use crate::{
    types::{Cli,Dropbox,Env,Http,View}
};

type Result<T> = std::result::Result<T,Error>;

#[derive(Debug)]
pub struct Controller {
    pub model: Http,
    pub view: View
}

impl Controller {
    // Model,View,Controller architecture
    pub async fn new() -> Result<Controller> {
        let env = Env::default();
        let view = View::new()?;
        let model = Http::default()
            .with_base_path(&env.api_path)
            .with_port(env.api_port);

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
        
        let username = view.prompt_public("username: ")?;
        let password = view.prompt_public("password: ")?;

        let session = match Session::start(&master_password).await? {
            SystemStatus::Locked => return Err(Error::SystemLocked),
            SystemStatus::Unlocked => {}
        };
        
        match &args.command {
            PrimaryCommand::Count(command) => command.run(path_opt, find_type_opt, view).await?,
            PrimaryCommand::Size(command) => command.run(path_opt, find_type_opt, view).await?,
            _ => return Err(Error::InvalidCommandLineArgument)
        };

        Ok(())
    }
}