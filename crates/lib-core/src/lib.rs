use std::fs::File;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result, Seek};

pub mod pitch;
pub mod samples;
pub mod test_helpers;
pub mod wav;
