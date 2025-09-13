use std::path::PathBuf;
use std::env;
use idropr_sdk_rust::{
    enums::{Auth,ApiResponse},
    types::Client as Model
};
use reqwest::Url;

use crate::{
    enums::{
        DroprError,
        PrimaryCommand
    }
};

use crate::types::{Cli,View};

type Result<T> = std::result::Result<T,DroprError>;

#[derive(Debug)]
pub struct Controller {
    pub model: Model,
    pub view: View
}

#[derive(Debug,Default)]
pub struct ControllerBuilder {
    pub model: Option<Model>,
    pub view: Option<View>
}

impl ControllerBuilder {
    // build and return the controller
    pub fn finish(self) -> Result<Controller> {
        let model = self.model.expect("missing data model");
        let view = self.view.expect("missing view controller");

        Ok(Controller {
            model,
            view
        })
    }

    // build the view controller
    pub fn with_view(mut self) -> Self {
        self.view = Some(View::default());
        self
    }

    // build the SDK CLient as the data model
    pub fn with_model(mut self, base_url: &Url, port: u16) -> Self {
        let model = Model::new(base_url, port);
        self.model = Some(model);
        self
    }
}

impl Controller {
    /// MVC architecture
    pub fn new() -> ControllerBuilder {
        ControllerBuilder::default()
    }

    /// exposes a utility to get the current directory for local operations when the file path is not explicity set
    #[inline]
    pub fn current_directory() -> Result<PathBuf> {
        let path_res = env::current_dir().map_err(|_e| DroprError::GetCurrentDirectory)?;

        Ok(path_res)
    }

    /// exposes a login utility to retreive an access token for all commands that require API privileges
    pub async fn login(&mut self) -> Result<()> {
        let username = self.view.prompt_public("username: ")?;
        let password = self.view.prompt_private("password: ")?;

        let login_response = self.model
            .sessions()
            .login(&username, &password).await?;

        let access_token = match login_response {
            ApiResponse::Ok(success) => success.data.access_token,
            ApiResponse::Error(e) => return Err(DroprError::ApiError(e.message))
        };

        let auth = Auth::Bearer(access_token);
        self.model.set_access_token(auth);

        Ok(())
    }

    /// exposes a logout utility to clean up sessions termination
    async fn logout(&self) -> Result<()> {
        self.model
            .sessions()
            .logout()
            .await?;

        println!("logged out");

        Ok(())
    }

    /// primary entry point for the program
    pub async fn run(&mut self, args: &Cli) -> Result<()> {
        type P = PrimaryCommand;

        match &args.command {
            P::Count(count) => count.run(args, self).await?,
            P::Size(size) => size.run(args, self).await?,
            P::Secret(secret) => secret.run(args, self).await?,
            _ => return Err(DroprError::InvalidCommandLineArgument)
        };

        match self.model.cfg().auth() {
            Auth::None => Ok(()),
            Auth::Bearer(_) => self.logout().await
        }
    }
}