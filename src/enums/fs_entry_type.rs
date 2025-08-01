#[derive(Clone,Debug,PartialEq)]
pub enum FsEntryType {
    Directory,
    File(String), // file name
    SymLink
}