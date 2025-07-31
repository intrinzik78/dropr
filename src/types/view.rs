use crate::enums::Error;

type Result<T> = std::result::Result<T,Error>;

#[derive(Debug)]
pub struct View {

}

impl View {
    pub fn new() -> Result<View> {
        let view = View {
            
        };

        Ok(view)
    }
}