mod attribute;
pub mod utils;

use std::path::{Path, PathBuf};

pub use attribute::*;

pub trait Device: Sized {
    fn open<P: AsRef<Path>>(device_node: P) -> anyhow::Result<Self>;
}

pub trait FindableDevice {
    fn find_device_nodes() -> Vec<PathBuf>;
}
