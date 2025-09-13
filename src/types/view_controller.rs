use std::io::{ stdout, stdin, Write };
use crate::enums::DroprError;

type Result<T> = std::result::Result<T,DroprError>;

#[derive(Debug,Default)]
pub struct View;

impl View {
    pub fn header(&self, m: &str) -> Result<&View> {
        println!("\n{m}");

        for _ in m.chars() {
            print!("-");
        }

        stdout().flush().map_err(|_| DroprError::FailedToFlushStdOut)?;
        println!();

        Ok(self)
    }

    pub fn end(&self) {
        println!();
    }

    pub fn print(&self, m: &str) -> Result<&View> {
        print!("{m}");
        stdout().flush().map_err(|_| DroprError::FailedToFlushStdOut)?;
        Ok(self)
    }

    pub fn println(&self, m: &str) -> &View {
        println!("{m}");
        self
    }

    pub fn prompt_public(&self, prompt: &str) -> Result<String> {
        let mut input:String = String::new();
        self.print(prompt)?;
        self.read_line(&mut input)?;

        Ok(input)
    }

    pub fn prompt_private(&self, prompt: &str) -> Result<String> {
        self.print(prompt)?;
        let token = rpassword::read_password().map_err(|_e| DroprError::StdReadPasswordError)?;

        Ok(token)
    }

    pub fn read_line(&self, s: &mut String) -> Result<()> {
        let _read_size = stdin().read_line(s).map_err(|_e| DroprError::StdReadLineError)?;
        let trimmed_size = s.trim_end_matches('\n')
            .trim_end_matches('\r')
            .len();
        
        s.truncate(trimmed_size);
        
        Ok(())
    }
}


