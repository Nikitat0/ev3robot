use std::error::Error as StdError;
use std::fmt::{Debug, Formatter};
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use tap::prelude::*;

pub trait DeviceAttribute: Sized {
    fn of_device(
        device_node: impl AsRef<Path>,
        name: &'static str,
    ) -> Result<Self>;
}

pub struct AttributeFile<T, const READABLE: bool, const WRITEABLE: bool> {
    file: File,
    phantom: PhantomData<*const T>,
}

type ReadableAttributeFile<T, const W: bool> = AttributeFile<T, true, W>;
type WriteableAttributeFile<T, const R: bool> = AttributeFile<T, R, true>;

pub type ReadOnlyAttributeFile<T> = ReadableAttributeFile<T, false>;
pub type WriteOnlyAttributeFile<T> = WriteableAttributeFile<T, false>;
pub type ReadWriteAttributeFile<T> = AttributeFile<T, true, true>;

impl<T, const R: bool, const W: bool> AttributeFile<T, R, W> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(AttributeFile {
            file: OpenOptions::new().read(R).write(W).open(path.as_ref())?,
            phantom: Default::default(),
        })
    }
}

impl<T, const R: bool, const W: bool> DeviceAttribute
    for AttributeFile<T, R, W>
{
    fn of_device(
        device_node: impl AsRef<Path>,
        name: &'static str,
    ) -> Result<Self> {
        Self::open(PathBuf::new().tap_mut(|it| {
            it.push(device_node);
            it.push(name);
        }))
    }
}

impl<T: FromStr, const W: bool> ReadableAttributeFile<T, W>
where
    T::Err: Into<Box<dyn StdError + Send + Sync>>,
{
    pub fn value(&mut self) -> Result<T> {
        let mut raw = String::new();
        self.file.seek(SeekFrom::Start(0))?;
        self.file.read_to_string(&mut raw)?;
        raw.trim_end()
            .parse()
            .map_err(|err: T::Err| Error::new(ErrorKind::InvalidData, err))
    }
}

impl<T: ToString, const R: bool> WriteableAttributeFile<T, R> {
    pub fn set_value(&mut self, value: T) -> Result<()> {
        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_all(value.to_string().as_bytes())?;
        Ok(())
    }
}

impl<T, const R: bool, const W: bool> Debug for AttributeFile<T, R, W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AttributeFile").field(&self.file).finish()
    }
}

impl<T: FromStr> DeviceAttribute for T
where
    T::Err: Into<Box<dyn StdError + Send + Sync>>,
{
    fn of_device(
        device_node: impl AsRef<Path>,
        name: &'static str,
    ) -> Result<Self> {
        ReadOnlyAttributeFile::of_device(device_node, name)?.value()
    }
}
