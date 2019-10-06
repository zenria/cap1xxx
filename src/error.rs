use std::fmt;
use std::fmt::{Debug, Display};

// Read Error
pub struct ReadError<T>(T);

//impl<R: Read> !Read for ReadError<R>;

impl<T> From<T> for ReadError<T> {
    fn from(read_error: T) -> Self {
        ReadError(read_error)
    }
}

impl<T: Display> Display for ReadError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "I2C ReadError: {}", self.0)
    }
}

impl<T: Debug> Debug for ReadError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ReadError {:?}", self.0)
    }
}

impl<T: std::error::Error> std::error::Error for ReadError<T> {}

// Write Error
pub struct WriteError<W>(W);

impl<W> From<W> for WriteError<W> {
    fn from(write_error: W) -> Self {
        WriteError(write_error)
    }
}

impl<T: Display> Display for WriteError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "I2C WriteError: {}", self.0)
    }
}
impl<T: Debug> Debug for WriteError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WriteError {:?}", self.0)
    }
}

impl<T: std::error::Error> std::error::Error for WriteError<T> {}

// Combined Read or Write Error
pub enum Error<R, W> {
    ReadError(ReadError<R>),
    WriteError(WriteError<W>),
    LedNumberOverflowError,
}

impl<R, W> std::error::Error for Error<R, W>
where
    R: std::error::Error,
    W: std::error::Error,
{
}

impl<R, W> Display for Error<R, W>
where
    R: Display,
    W: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::ReadError(e) => std::fmt::Display::fmt(&e, f),
            Error::WriteError(e) => std::fmt::Display::fmt(&e, f),
            Error::LedNumberOverflowError => write!(f, "Invalid led number!"),
        }
    }
}
impl<R, W> Debug for Error<R, W>
where
    R: Debug,
    W: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::ReadError(e) => std::fmt::Debug::fmt(&e, f),
            Error::WriteError(e) => std::fmt::Debug::fmt(&e, f),
            Error::LedNumberOverflowError => write!(f, "LedNumberOverflowError"),
        }
    }
}

impl<R, W> From<ReadError<R>> for Error<R, W> {
    fn from(e: ReadError<R>) -> Self {
        Error::ReadError(e)
    }
}
impl<R, W> From<WriteError<W>> for Error<R, W> {
    fn from(e: WriteError<W>) -> Self {
        Error::WriteError(e)
    }
}
