use std::fs::File;
use std::io::{Error, Result, prelude::*};
use std::path::Path;

struct Wav {
    sample_rate: u32,
    channels: u16,
}

fn read_wav<P: AsRef<Path>>(path: P) -> Result<Wav> {
    let mut file = File::open(path)?;
    let mut header = [0u8; 12];

    file.read_exact(&mut header)?;

    if &header[0..4] != b"RIFF" || &header[8..12] != b"WAVE" {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid WAV input",
        ));
    }

    let mut fmt_header = [0u8; 16];
    file.read_exact(&mut fmt_header)?;

    let sample_rate = u32::from_le_bytes(fmt_header[12..16].try_into().unwrap());
    let channels = u16::from_le_bytes(fmt_header[10..12].try_into().unwrap());

    Ok(Wav {
        sample_rate: sample_rate,
        channels: channels,
    })
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;

    use super::*;

    #[test]
    fn file_exists() {
        let path = "exists.wav";
        let mut file = File::create(path).unwrap();
        let mut header = Vec::new();
        header.extend_from_slice(b"RIFFxxxxWAVE");
        header.extend_from_slice(&[0u8; 16]);
        file.write_all(&header).unwrap();

        let res = read_wav(path);

        remove_file(path).unwrap();
        assert!(res.is_ok());
    }

    #[test]
    fn file_not_exists() {
        let res = read_wav("not_exists.wav");
        assert!(res.is_err());
    }

    #[test]
    fn valid_wav() {
        let path = "valid.wav";
        let mut file = File::create(path).unwrap();
        let mut header = Vec::new();
        header.extend_from_slice(b"RIFFxxxxWAVE");
        header.extend_from_slice(&[0u8; 16]);
        file.write_all(&header).unwrap();

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

    #[test]
    fn extract_sample_rate() {
        let path = "sample_rate.wav";
        let mut file = File::create(path).unwrap();
        let mut header = Vec::new();
        header.extend_from_slice(b"RIFF");
        header.extend_from_slice(&[0u8; 4]);
        header.extend_from_slice(b"WAVE");

        header.extend_from_slice(b"fmt ");
        header.extend_from_slice(&[0u8; 4]);
        header.extend_from_slice(&[0u8; 2]);
        header.extend_from_slice(&[0u8; 2]);
        header.extend_from_slice(&44_100u32.to_le_bytes());

        file.write_all(&header).unwrap();
        file.sync_all().unwrap();

        let wav = read_wav(path).unwrap();

        assert_eq!(wav.sample_rate, 44100);
        remove_file(path).unwrap();
    }

    #[test]
    fn wav_channels() {
        let path = "channels.wav";
        let mut file = File::create(path).unwrap();
        let mut header = Vec::new();
        header.extend_from_slice(b"RIFF");
        header.extend_from_slice(&[0u8; 4]);
        header.extend_from_slice(b"WAVE");

        header.extend_from_slice(b"fmt ");
        header.extend_from_slice(&[0u8; 4]);
        header.extend_from_slice(&[0u8; 2]);
        header.extend_from_slice(&2u16.to_le_bytes());
        header.extend_from_slice(&44_100u32.to_le_bytes());

        file.write_all(&header).unwrap();

        let wav = read_wav(path).unwrap();

        assert_eq!(wav.channels, 2);
        remove_file(path).unwrap();
    }
}
