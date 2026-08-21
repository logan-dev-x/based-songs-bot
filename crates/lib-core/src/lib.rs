use std::fs::File;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result, Seek};

pub mod pitch;
pub mod processing;
pub mod samples;
pub mod wav;

#[cfg(test)]
mod test_helpers;
