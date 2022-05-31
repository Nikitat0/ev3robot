use std::error;
use std::fmt::Display;

use thiserror::Error;

pub type ConnectionResult<T> = Result<T, ConnectionError>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ConnectionError {
    #[error("There is no device with name {name}")]
    NotExist { name: String },
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl ConnectionError {
    pub fn new_unexpected<E>(err: E) -> ConnectionError
    where
        E: error::Error + Send + Sync + 'static,
    {
        ConnectionError::Unexpected(anyhow::Error::new(err))
    }

    pub fn new_unexpected_with_context<E, C>(
        err: E,
        context: C,
    ) -> ConnectionError
    where
        E: error::Error + Send + Sync + 'static,
        C: Display + Send + Sync + 'static,
    {
        ConnectionError::Unexpected(anyhow::Error::new(err).context(context))
    }
}
