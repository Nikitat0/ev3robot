use std::error::Error as StdError;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};
use std::marker::PhantomData;
use std::path::Path;
use std::str::FromStr;

pub struct Attribute<T, const READABLE: bool, const WRITEABLE: bool> {
    file: File,
    phantom: PhantomData<*const T>,
}

type ReadableAttribute<T, const W: bool> = Attribute<T, true, W>;
type WriteableAttribute<T, const R: bool> = Attribute<T, R, true>;

pub type ReadOnlyAttribute<T> = ReadableAttribute<T, false>;
pub type WriteOnlyAttribute<T> = WriteableAttribute<T, false>;
pub type ReadWriteAttribute<T> = Attribute<T, true, true>;

impl<T, const R: bool, const W: bool> Attribute<T, R, W> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Attribute {
            file: OpenOptions::new().read(R).write(W).open(path.as_ref())?,
            phantom: Default::default(),
        })
    }
}

impl<T: FromStr, const W: bool> ReadableAttribute<T, W>
where
    T::Err: Into<Box<dyn StdError + Send + Sync>>,
{
    pub fn get(&mut self) -> Result<T> {
        let mut raw = String::new();
        self.file.seek(SeekFrom::Start(0))?;
        self.file.read_to_string(&mut raw)?;
        raw.trim_end()
            .parse()
            .map_err(|err: T::Err| Error::new(ErrorKind::InvalidData, err))
    }
}

impl<T: ToString, const R: bool> WriteableAttribute<T, R> {
    pub fn set(&mut self, value: T) -> Result<()> {
        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_all(value.to_string().as_bytes())?;
        Ok(())
    }
}
