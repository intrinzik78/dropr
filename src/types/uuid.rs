use crate::enums::{ Error,UUIDType };

type Result<T> = std::result::Result<T,Error>;

#[derive(Debug)]
pub struct UUID {
    pub id: String 
}

impl UUID {
    pub fn new(_uuid_type: UUIDType, _length: u8) -> Result<()> {
        Ok(())
    }
}