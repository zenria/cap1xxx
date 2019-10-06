use std::fmt;
use std::fmt::{Debug, Display};

// Read Error
#[derive(Debug)]
pub struct ReadError<T: Debug>(T);

//impl<R: Read> !Read for ReadError<R>;

impl<T: Debug> From<T> for ReadError<T> {
    fn from(read_error: T) -> Self {
        ReadError(read_error)
    }
}

impl<T: Display + Debug> Display for ReadError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "I2C ReadError: {}", self.0)
    }
}

impl<T: std::error::Error> std::error::Error for ReadError<T> {}

// Write Error
#[derive(Debug)]
pub struct WriteError<W: Debug>(W);

impl<W: Debug> From<W> for WriteError<W> {
    fn from(write_error: W) -> Self {
        WriteError(write_error)
    }
}

impl<T: Display + Debug> Display for WriteError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "I2C WriteError: {}", self.0)
    }
}

impl<T: std::error::Error> std::error::Error for WriteError<T> {}

// Combined Read or Write Error
#[derive(Debug)]
pub enum Error<R, W>
where
    R: Debug,
    W: Debug,
{
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
    R: Debug + Display,
    W: Debug + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::ReadError(e) => std::fmt::Display::fmt(&e, f),
            Error::WriteError(e) => std::fmt::Display::fmt(&e, f),
            Error::LedNumberOverflowError => write!(f, "Invalid led number!"),
        }
    }
}

impl<R, W> From<ReadError<R>> for Error<R, W>
where
    R: Debug,
    W: Debug,
{
    fn from(e: ReadError<R>) -> Self {
        Error::ReadError(e)
    }
}
impl<R, W> From<WriteError<W>> for Error<R, W>
where
    R: Debug,
    W: Debug,
{
    fn from(e: WriteError<W>) -> Self {
        Error::WriteError(e)
    }
}
