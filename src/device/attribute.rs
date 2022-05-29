use std::cell::RefCell;
use std::error::Error as StdError;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub trait Attribute: Sized {
    fn of_device(
        device_class: impl AsRef<OsStr>,
        device_name: impl AsRef<OsStr>,
        attr_name: &'static str,
    ) -> Result<Self>;
}

pub struct AttributeFile<T, const READABLE: bool, const WRITEABLE: bool> {
    file: RefCell<File>,
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
            file: RefCell::new(
                OpenOptions::new().read(R).write(W).open(path.as_ref())?,
            ),
            phantom: Default::default(),
        })
    }
}

impl<T, const R: bool, const W: bool> Attribute for AttributeFile<T, R, W> {
    fn of_device(
        device_class: impl AsRef<OsStr>,
        device_name: impl AsRef<OsStr>,
        attr_name: &'static str,
    ) -> Result<Self> {
        let mut path = PathBuf::from("/sys/class");
        path.push(device_class.as_ref());
        path.push(device_name.as_ref());
        path.push(attr_name);
        Self::open(path)
    }
}

impl<T: FromStr, const W: bool> ReadableAttributeFile<T, W>
where
    T::Err: Into<Box<dyn StdError + Send + Sync>>,
{
    pub fn value(&mut self) -> Result<T> {
        let mut file = self.file.borrow_mut();
        let mut raw = String::new();
        file.seek(SeekFrom::Start(0))?;
        file.read_to_string(&mut raw)?;
        raw.trim_end()
            .parse()
            .map_err(|err: T::Err| Error::new(ErrorKind::InvalidData, err))
    }
}

impl<T: ToString, const R: bool> WriteableAttributeFile<T, R> {
    pub fn set_value(&mut self, value: T) -> Result<()> {
        let mut file = self.file.borrow_mut();
        file.seek(SeekFrom::Start(0))?;
        file.write_all(value.to_string().as_bytes())?;
        Ok(())
    }
}

impl<T: FromStr> Attribute for T
where
    T::Err: Into<Box<dyn StdError + Send + Sync>>,
{
    fn of_device(
        device_class: impl AsRef<OsStr>,
        device_name: impl AsRef<OsStr>,
        attr_name: &'static str,
    ) -> Result<Self> {
        ReadOnlyAttributeFile::of_device(device_class, device_name, attr_name)?
            .value()
    }
}
