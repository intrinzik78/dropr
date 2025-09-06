use std::collections::HashMap;

use dotenv;

// manages importing and testing of the .env file
#[derive(Debug)]
pub struct Env {
    pub api_path: String,
    pub api_port: u16
}

impl Default for Env {
    fn default() -> Self {
        // load values
        let env:HashMap<String,String> = dotenv::vars().collect();

        // extracts env vars
        let api_path = env.get("API_PATH")
            .expect("API_PATH not found in .env")
            .to_owned();

        let api_port: u16 = env.get("API_PORT")
            .expect("API_PORT not found in .env")
            .to_owned()
            .parse()
            .expect("could not parse API_PORT field in .env");

        Env {
            api_path,
            api_port
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_env_builder() {
        // manually construct Env, will fail on missing values
        let manual_env = Env {
            api_path: String::from("api_path"),
            api_port: 3000
        };

        // test function calls return correct data
        assert_eq!(manual_env.api_path, String::from("api_path"));
        assert_eq!(manual_env.api_port, 3000);
    }
}