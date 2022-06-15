use std::error::Error as StdError;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::string::ToString;

use tap::prelude::*;

pub trait DeviceAttribute: Sized {
    fn of_device(
        device_node: impl AsRef<Path>,
        name: &'static str,
    ) -> Result<Self>;
}

#[derive(Debug)]
pub struct AttributeFile<const READABLE: bool, const WRITEABLE: bool>(File);

type ReadableAttributeFile<const W: bool> = AttributeFile<true, W>;
type WriteableAttributeFile<const R: bool> = AttributeFile<R, true>;

pub type ReadOnlyAttributeFile = ReadableAttributeFile<false>;
pub type WriteOnlyAttributeFile = WriteableAttributeFile<false>;
pub type ReadWriteAttributeFile = AttributeFile<true, true>;

impl<const R: bool, const W: bool> AttributeFile<R, W> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(AttributeFile(
            OpenOptions::new().read(R).write(W).open(path.as_ref())?,
        ))
    }

    fn file(&mut self) -> &mut File {
        &mut self.0
    }
}

impl<const R: bool, const W: bool> DeviceAttribute for AttributeFile<R, W> {
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

impl<const W: bool> ReadableAttributeFile<W> {
    pub fn value<T: FromStr>(&mut self) -> Result<T>
    where
        T::Err: Into<Box<dyn StdError + Send + Sync>>,
    {
        let mut raw = String::new();
        self.file().seek(SeekFrom::Start(0))?;
        self.file().read_to_string(&mut raw)?;
        raw.trim_end()
            .parse()
            .map_err(|err: T::Err| Error::new(ErrorKind::InvalidData, err))
    }
}

impl<const R: bool> WriteableAttributeFile<R> {
    pub fn set_value<T: ToString>(&mut self, value: T) -> Result<()> {
        self.file().seek(SeekFrom::Start(0))?;
        self.file().write_all(value.to_string().as_bytes())?;
        Ok(())
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
