use thiserror::Error;

pub type ConnectionResult<T> = Result<T, ConnectionError>;

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("There is no device with name {name}")]
    NotExist { name: String },
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}
