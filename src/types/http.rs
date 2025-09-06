use serde::Serializer;

use crate::{
    enums::{Error,Method}
};

type Result<T> = std::result::Result<T,Error>;

#[derive(Debug, Default)]
pub struct Http {
    pub api_path: Option<String>,
    pub api_port: Option<u16>,
}

impl Http {
    pub fn with_base_path(mut self, path: &str) -> Self {
        match path.is_empty() {
            true => self.api_path = None,
            false => self.api_path = Some(path.to_owned())
        };

        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.api_port = Some(port);

        self
    }

    pub fn send(&self, _data: Option<impl Serializer>) -> Result<()> {
        let _base_path = match &self.api_path {
            Some(p) => p,
            None => return Err(Error::NoRemotePathSpecified)
        };

        let _port = match &self.api_path {
            Some(p) => p,
            None => return Err(Error::NoValidPortProvided)
        };

        Ok(())
    }
}