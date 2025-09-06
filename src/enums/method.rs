#[repr(u8)]
#[derive(Clone,Debug,PartialEq)]
pub enum Method {
    Delete  = 0,
    Get     = 1,
    Patch   = 2,
    Post    = 3
}