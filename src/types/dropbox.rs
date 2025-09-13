use serde::{Deserialize,Serialize};

use crate::enums::DroprError;

type Result<T> = std::result::Result<T,DroprError>;


#[derive(Debug,Serialize)]
pub struct CreateBucketRequest {

}

#[derive(Debug,Deserialize)]
pub struct CreateBucketResponse {

}

#[derive(Debug,Serialize)]
pub struct CreateFileRequest {

}

#[derive(Debug,Deserialize)]
pub struct CreateFileResponse {
    
}


#[derive(Debug)]
pub struct Dropbox<'a> {
    pub key: &'a str,
    pub secret: &'a str
}


impl <'a> Dropbox <'a> {
    pub fn new(key: &'a str, secret: &'a str) -> Dropbox<'a> {
        Dropbox {
            key,
            secret
        }
    }

    // create: (/create) https://api.dropboxapi.com/2/files/create_folder_v2
    pub fn create_bucket() -> Result<()> {
        todo!()
    }
    
    // delete: (/delete) https://api.dropboxapi.com/2/files/delete_v2
    pub fn delete_bucket() -> Result<()> {
        todo!()
    }

    // list: (/list) https://api.dropboxapi.com/2/file_requests/list_v2
    pub fn list_buckets() -> Result<()> {
        todo!()
    }

    // (/upda) https://api.dropboxapi.com/2/file_properties/properties/update
    pub fn rename_bucket() -> Result<()> {
        todo!()
    }


    // (/upload) https://content.dropboxapi.com/2/files/upload
    pub fn upload_file() -> Result<()> {
        todo!()
    }

}