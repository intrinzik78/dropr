use std::io::{ stdout, Write };
use crate::enums::Error;

type Result<T> = std::result::Result<T,Error>;

#[derive(Debug)]
pub struct View;

impl View {
    pub fn new() -> Result<View> {
        let view = View {};

        Ok(view)
    }

    pub fn header(&self, m: &str) -> Result<&View> {
        println!("\n{}",m);

        for _ in m.chars() {
            print!("-");
        }

        print!("\n");
        stdout().flush().map_err(|_| Error::FailedToFlushStdOut)?;

        Ok(&self)
    }

    pub fn end(&self) {
        println!("");
    }

    pub fn print(&self, m: &str) -> Result<&View> {
        print!("{}",m);
        stdout().flush().map_err(|_| Error::FailedToFlushStdOut)?;
        Ok(&self)
    }

    pub fn println(&self, m: &str) -> &View {
        println!("{}",m);
        &self
    }

    pub fn prompt_public(&self, _m: &str) -> Result<String> {
        todo!()
    }

    pub fn prompt_private(&self, _m: &str) -> Result<String> {
        todo!()
    }
}