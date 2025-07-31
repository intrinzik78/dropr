use crate::enums::Error;

type Result<T> = std::result::Result<T,Error>;

#[derive(Debug)]
pub struct DatabaseConnection {

}

impl DatabaseConnection {
    pub async fn new() -> Result<DatabaseConnection> {
        let database = DatabaseConnection{};

        Ok(database)
    }
}