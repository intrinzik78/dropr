use derive_more::From;
use sqlx;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Sqlx(sqlx::Error)
}