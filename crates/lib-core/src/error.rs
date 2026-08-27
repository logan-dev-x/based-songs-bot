use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    InvalidMp3,
    InvalidChannels,
    UnsupportedFormat,
    Decode(String),
    Encode(String),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
