mod attribute;
mod error;

use std::ffi::OsStr;

pub use attribute::*;
pub use error::*;

pub trait Device: Sized {
    const CLASS: &'static str;

    fn try_connect<S: AsRef<OsStr>>(name: S) -> ConnectionResult<Self>;
}
