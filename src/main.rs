use clap::Parser;
use reqwest::Url;

pub mod types;
pub mod enums;
pub mod traits;

use enums::DroprError;
use types::{ Cli, Controller };

type Result<T> = std::result::Result<T,DroprError>;

#[tokio::main]
async fn main() -> Result<()> {
    // parse command line args, terminating the program on parse error and displaying --help instead
    let cli_arguments = Cli::parse();

    // extract env args / params
    let env = types::Env::default();
    let base_url = Url::parse(&env.api_path).expect("valid api base url");
    let port = env.api_port;
    
     // build controller with view & model
    let mut controller = Controller::new()
        .with_model(&base_url, port)
        .with_view()
        .finish()?;
   
    // execute the program
    match controller.run(&cli_arguments).await {
        Ok(_) => controller.view.println("[finished ok]"),
        Err(e) => controller.view.println(&e.to_string())
    };

    Ok(())
}
