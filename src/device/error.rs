use std::error;
use std::ffi::OsStr;
use std::fmt::Display;
use std::path::PathBuf;

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

pub fn is_device_exists<S: AsRef<OsStr>>(
    class: &'static str,
    device_name: S,
) -> bool {
    let mut path = PathBuf::from("/sys/class");
    path.push(class);
    path.push(device_name.as_ref());
    path.exists()
}

pub fn check_device_exists<S: AsRef<OsStr>>(
    class: &'static str,
    device_name: S,
) -> ConnectionResult<()> {
    is_device_exists(class, device_name.as_ref()).then(|| ()).ok_or(
        ConnectionError::NotExist {
            name: device_name.as_ref().to_string_lossy().to_string(),
        },
    )
}
