use std::fs::File;
use std::io::{Error, Result, prelude::*};
use std::path::Path;

fn read_wav<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut file = File::open(path)?;
    let mut header = [0u8; 12];

    file.read_exact(&mut header)?;

    if &header[0..4] != b"RIFF" || &header[8..12] != b"WAVE" {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid WAV input",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;

    use super::*;

    #[test]
    fn file_exists() {
        let path = "exists.wav";
        let mut file = File::create(path).unwrap();
        file.write_all(b"RIFFxxxxWAVE").unwrap();

        let res = read_wav(path);

        remove_file(path).unwrap();
        assert!(res.is_ok());
    }

    #[test]
    fn file_not_exists() {
        let res = read_wav("foo.wav");
        assert!(res.is_err());
    }

    #[test]
    fn valid_wav() {
        let path = "valid.wav";
        let mut file = File::create(path).unwrap();
        file.write_all(b"RIFFxxxxWAVE").unwrap();

        let res = read_wav(path);

        remove_file(path).unwrap();
        assert!(res.is_ok());
    }

    #[test]
    fn invalid_wav() {
        let path = "invalid.wav";
        let mut file = File::create(path).unwrap();
        file.write_all(b"xxxxxxxxxxxx").unwrap();

        let res = read_wav(path);

        remove_file(path).unwrap();
        assert!(res.is_err());
    }
}
