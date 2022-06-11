use std::borrow::Borrow;
use std::fmt::{self, Debug};

use thiserror::Error;

pub trait Findable: Sized {
    fn find() -> Result<Self>;
}

pub trait FindableBy<T: ?Sized>: Sized {
    fn find_by<U: Borrow<T>>(value: U) -> Result<Self>;
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error)]
pub enum Error {
    #[error("not found")]
    NotFound,
    #[error("found multiple")]
    FoundMultiple,
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotFound => f.debug_tuple("NotFound").finish(),
            Error::FoundMultiple => f.debug_tuple("FoundMultiple").finish(),
            Error::Unexpected(err) => Debug::fmt(err, f),
        }
    }
}

pub fn find_in<T, I: IntoIterator<Item = T>>(iterable: I) -> Result<T> {
    let mut iter = iterable.into_iter();
    let found = iter.next().ok_or(Error::NotFound)?;
    iter.next().is_none().then(|| found).ok_or(Error::FoundMultiple)
}
