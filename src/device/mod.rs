mod attribute;

use std::path::Path;

pub use attribute::*;

pub trait Device: Sized {
    fn open<P: AsRef<Path>>(device_node: P) -> anyhow::Result<Self>;
}
