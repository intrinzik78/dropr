use clap::Subcommand;
use idropr_sdk_rust::enums::ApiResponse;

use crate::{
    enums::DroprError,
    types::{Cli, Controller}
};

type Result<T> = std::result::Result<T,DroprError>;

#[derive(Clone,Debug,PartialEq,Subcommand)]
pub enum SecretCommand {
    // Create a new api secret on database
    New,
    // Delete an existing api secret from the database
    Delete
}

impl SecretCommand {
    /// creates a new database secret
    pub async fn create_new_secret(&self, controller: &mut Controller) -> Result<()> {
        // set access token
        let () = controller.login().await?;

        // take all input
        controller.view.header("Api Secret")?;

        let name = controller.view.prompt_public("Api name (required): ")?;
        let description = controller.view.prompt_public("Api description (required): ")?;
        let api_key_input = controller.view.prompt_public("Api key (optional): ")?;
        let api_secret_input = controller.view.prompt_public("Api secret (optional): ")?;


        /*
        
            BEGIN HERE BY BUILDING A VIEW CONTROLLER Y/N PROMPT RESPONDER

            // let confirmation = match controller.view.prompt_public("Confirm: y/N")?.to_ascii_lowercase() {
    
            // };
        
         */

        // reformate api key
        let api_key_opt = match api_key_input.is_empty() {
            true => None,
            false => Some(api_key_input)
        };

        // reformat api secret
        let api_secret_opt = match api_secret_input.is_empty() {
            true => None,
            false => Some(api_secret_input)
        };

        // run query, get the SDK response
        let sdk_response = controller.model
            .secrets()
            .create_secret(&name, &description, api_key_opt.as_ref(), api_secret_opt.as_ref())
            .await;

        // extract server response from SDK response
        let response = match sdk_response {
            Ok(r) => r,
            Err(e) => {
                println!("{}",e);
                return Err(DroprError::SDKError(e));
            }
        };

         // handle responses
        match response {
            ApiResponse::Ok(s) => {
                if s.code == 200 {
                   controller.view.println("api added to database");
                } else {
                   controller.view.header("Error")?;
                   let m = format!("[{}] {}",s.code,s.message);
                   controller.view.println(&m);
                }
            },
            ApiResponse::Error(e) => {
                println!("[{}] {}",e.code,e.message);
            }
        };

        Ok(())
    }

    pub async fn run(&self, _args: &Cli, controller: &mut Controller) -> Result<()> {
        let _result = match self {
            Self::New => self.create_new_secret(controller).await?,
            _ => todo!()
        };

        Ok(())
    }
}