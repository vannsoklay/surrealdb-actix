use crate::utils::response;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Fail to get Ctx '{0}'")]
    CtxFail(&'static str),

    #[error("Value not of type '{0}'")]
    XValueNotOfType(&'static str),

    #[error("Property '{0}' not found")]
    XPropertyNotFound(String),

    #[error("Fail to create. Cause: {0}")]
    StoreFailToCreate(String),

    #[error(transparent)]
    Surreal(#[from] surrealdb::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
pub struct W<T>(pub T);
pub type Response<T> = std::result::Result<T, response::Error>;