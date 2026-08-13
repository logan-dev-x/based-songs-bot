use std::fs::File;
use std::io::{Error, Result, prelude::*};
use std::path::Path;

struct Wav {
    sample_rate: u32,
    channels: u16,
    bits_per_sample: u16,
}

struct Chunk {
    id: [u8; 4],
    data: Vec<u8>,
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

    let chunk = read_chunk(&mut file)?;

    if chunk.id != *b"fmt " {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Expected fmt chunk",
        ));
    }

    let channels = u16::from_le_bytes(chunk.data[2..4].try_into().unwrap());
    let sample_rate = u32::from_le_bytes(chunk.data[4..8].try_into().unwrap());
    let bits_per_sample = u16::from_le_bytes(chunk.data[14..16].try_into().unwrap());

    Ok(Wav {
        sample_rate,
        channels,
        bits_per_sample,
    })
}

fn read_chunk(file: &mut File) -> Result<Chunk> {
    let mut id = [0u8; 4];
    file.read_exact(&mut id)?;

    let mut size_bytes = [0u8; 4];
    file.read_exact(&mut size_bytes)?;
    let size = u32::from_le_bytes(size_bytes);

    let mut data = vec![0u8; size as usize];
    file.read_exact(&mut data)?;

    Ok(Chunk { id, data })
}

#[cfg(test)]
mod tests {
    use std::fs::{OpenOptions, remove_file};

    use super::*;

    #[test]
    fn file_exists() {
        let path = "exists.wav";
        let mut file = File::create(path).unwrap();
        let mut header = Vec::new();
        header.extend_from_slice(b"RIFFxxxxWAVEfmt ");
        header.extend_from_slice(&16u32.to_le_bytes());
        header.extend_from_slice(&[0u8; 20]);
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
        header.extend_from_slice(b"RIFFxxxxWAVEfmt ");
        header.extend_from_slice(&16u32.to_le_bytes());
        header.extend_from_slice(&[0u8; 20]);
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
        header.extend_from_slice(&16u32.to_le_bytes());
        header.extend_from_slice(&[0u8; 2]);
        header.extend_from_slice(&[0u8; 2]);
        header.extend_from_slice(&44_100u32.to_le_bytes());
        header.extend_from_slice(&[0u8; 4]);
        header.extend_from_slice(&[0u8; 2]);
        header.extend_from_slice(&[0u8; 2]);

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
        header.extend_from_slice(&16u32.to_le_bytes());
        header.extend_from_slice(&[0u8; 2]);
        header.extend_from_slice(&2u16.to_le_bytes());
        header.extend_from_slice(&44_100u32.to_le_bytes());
        header.extend_from_slice(&[0u8; 4]);
        header.extend_from_slice(&[0u8; 2]);
        header.extend_from_slice(&[0u8; 2]);

        file.write_all(&header).unwrap();

        let wav = read_wav(path).unwrap();

        assert_eq!(wav.channels, 2);
        remove_file(path).unwrap();
    }

    #[test]
    fn wav_bits_per_sample() {
        let path = "bits_per_sample.wav";
        let mut file = File::create(path).unwrap();
        let mut header = Vec::new();
        header.extend_from_slice(b"RIFF");
        header.extend_from_slice(&[0u8; 4]);
        header.extend_from_slice(b"WAVE");

        header.extend_from_slice(b"fmt ");
        header.extend_from_slice(&16u32.to_le_bytes());
        header.extend_from_slice(&[0u8; 2]);
        header.extend_from_slice(&2u16.to_le_bytes());
        header.extend_from_slice(&44_100u32.to_le_bytes());
        header.extend_from_slice(&[0u8; 4]);
        header.extend_from_slice(&[0u8; 2]);
        header.extend_from_slice(&16u16.to_le_bytes());

        file.write_all(&header).unwrap();

        let wav = read_wav(path).unwrap();

        assert_eq!(wav.bits_per_sample, 16);
        remove_file(path).unwrap();
    }

    #[test]
    fn read_chunk_fmt() {
        let path = "chunk.wav";
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        let mut header = Vec::new();
        header.extend_from_slice(b"fmt ");
        header.extend_from_slice(&4u32.to_le_bytes());
        header.extend_from_slice(&[1, 2, 3, 4]);

        file.write_all(&header).unwrap();
        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let chunk = read_chunk(&mut file).unwrap();

        assert_eq!(chunk.id, *b"fmt ");
        assert_eq!(chunk.data, vec![1, 2, 3, 4]);

        remove_file(path).unwrap();
    }
}
