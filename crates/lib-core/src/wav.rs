use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::path::Path;

pub struct Wav {
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
    pub data: Vec<u8>,
}

pub struct Chunk {
    id: [u8; 4],
    data: Vec<u8>,
}

pub fn write_wav<P: AsRef<Path>>(wav: &Wav, path: P) -> Result<()> {
    let mut file = File::create(path)?;
    let mut header = Vec::new();

    header.extend_from_slice(b"RIFF");
    header.extend_from_slice(&0u32.to_le_bytes());
    header.extend_from_slice(b"WAVE");

    file.write_all(&header);

    Ok(())
}

pub fn read_wav<P: AsRef<Path>>(path: P) -> Result<Wav> {
    let mut file = File::open(path)?;
    let mut header = [0u8; 12];

    file.read_exact(&mut header)?;

    if &header[0..4] != b"RIFF" || &header[8..12] != b"WAVE" {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid WAV input"));
    }

    let mut data = None;
    let mut channels = None;
    let mut sample_rate = None;
    let mut bits_per_sample = None;

    loop {
        let chunk = read_chunk(&mut file)?;

        if chunk.id == *b"data" {
            data = Some(chunk.data.clone());
        }

        if chunk.id == *b"fmt " {
            if chunk.data.len() < 16 {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid fmt chunk"));
            }

            if u16::from_le_bytes(chunk.data[0..2].try_into().unwrap()) != 1 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Only PCM WAV is supported",
                ));
            }

            bits_per_sample = Some(u16::from_le_bytes(chunk.data[14..16].try_into().unwrap()));
            if bits_per_sample.unwrap() != 16 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Only 16-bit is supported",
                ));
            }

            channels = Some(u16::from_le_bytes(chunk.data[2..4].try_into().unwrap()));
            sample_rate = Some(u32::from_le_bytes(chunk.data[4..8].try_into().unwrap()));
        }

        if data.is_some()
            && channels.is_some()
            && sample_rate.is_some()
            && bits_per_sample.is_some()
        {
            return Ok(Wav {
                sample_rate: sample_rate.unwrap(),
                channels: channels.unwrap(),
                bits_per_sample: bits_per_sample.unwrap(),
                data: data.unwrap(),
            });
        }
    }
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
    use super::*;
    use crate::test_helpers::*;
    use std::io::Write;

    #[test]
    fn file_exists() {
        let mut header = Vec::new();
        write_wav_header(&mut header);
        write_fmt_chunk(&mut header);
        write_data_chunk(&mut header);
        let ctx: TestContext = setup(&mut header);

        let res = read_wav(&ctx.path);

        teardown(ctx.path);
        assert!(res.is_ok());
    }

    #[test]
    fn file_not_exists() {
        let res = read_wav("not_exists.wav");
        assert!(res.is_err());
    }

    #[test]
    fn valid_wav() {
        let mut header = Vec::new();
        write_wav_header(&mut header);
        write_fmt_chunk(&mut header);
        write_data_chunk(&mut header);
        let ctx: TestContext = setup(&mut header);

        let res = read_wav(&ctx.path);

        teardown(ctx.path);
        assert!(res.is_ok());
    }

    #[test]
    fn invalid_wav() {
        let mut header = Vec::new();
        header.extend_from_slice(&[0u8; 12]);
        let ctx: TestContext = setup(&mut header);

        let res = read_wav(&ctx.path);

        teardown(ctx.path);
        assert!(res.is_err());
    }

    #[test]
    fn read_chunk_fmt() {
        let mut header = Vec::new();
        header.extend_from_slice(b"fmt ");
        header.extend_from_slice(&4u32.to_le_bytes());
        header.extend_from_slice(&[1, 2, 3, 4]);
        write_data_chunk(&mut header);
        let mut ctx: TestContext = setup(&mut header);

        let chunk = read_chunk(&mut ctx.file).unwrap();

        teardown(ctx.path);
        assert_eq!(chunk.id, *b"fmt ");
        assert_eq!(chunk.data, vec![1, 2, 3, 4]);
    }

    #[test]
    fn wav_channels() {
        let mut header = Vec::new();
        write_wav_header(&mut header);
        write_data_chunk(&mut header);
        header.extend_from_slice(b"fmt ");
        header.extend_from_slice(&16u32.to_le_bytes());
        header.extend_from_slice(&1u16.to_le_bytes());
        header.extend_from_slice(&2u16.to_le_bytes());
        header.extend_from_slice(&[0u8; 10]);
        header.extend_from_slice(&16u16.to_le_bytes());
        let ctx: TestContext = setup(&mut header);

        let wav = read_wav(&ctx.path).unwrap();

        teardown(ctx.path);
        assert_eq!(wav.channels, 2);
    }

    #[test]
    fn extract_sample_rate() {
        let mut header = Vec::new();
        write_wav_header(&mut header);
        write_data_chunk(&mut header);
        header.extend_from_slice(b"fmt ");
        header.extend_from_slice(&16u32.to_le_bytes());
        header.extend_from_slice(&1u16.to_le_bytes());
        header.extend_from_slice(&[0u8; 2]);
        header.extend_from_slice(&44_100u32.to_le_bytes());
        header.extend_from_slice(&[0u8; 6]);
        header.extend_from_slice(&16u16.to_le_bytes());
        let ctx: TestContext = setup(&mut header);

        let wav = read_wav(&ctx.path).unwrap();

        teardown(ctx.path);
        assert_eq!(wav.sample_rate, 44100);
    }

    #[test]
    fn wav_bits_per_sample() {
        let mut header = Vec::new();
        write_wav_header(&mut header);
        write_data_chunk(&mut header);
        header.extend_from_slice(b"fmt ");
        header.extend_from_slice(&16u32.to_le_bytes());
        header.extend_from_slice(&1u16.to_le_bytes());
        header.extend_from_slice(&[0u8; 12]);
        header.extend_from_slice(&16u16.to_le_bytes());
        let ctx: TestContext = setup(&mut header);

        let wav = read_wav(&ctx.path).unwrap();

        teardown(ctx.path);
        assert_eq!(wav.bits_per_sample, 16);
    }

    #[test]
    fn find_fmt_after_junk() {
        let mut header = Vec::new();
        write_wav_header(&mut header);
        write_junk_chunk(&mut header);
        header.extend_from_slice(b"fmt ");
        header.extend_from_slice(&16u32.to_le_bytes());
        header.extend_from_slice(&1u16.to_le_bytes());
        header.extend_from_slice(&[0u8; 2]);
        header.extend_from_slice(&44_100u32.to_le_bytes());
        header.extend_from_slice(&[0u8; 6]);
        header.extend_from_slice(&16u16.to_le_bytes());
        write_data_chunk(&mut header);
        let ctx: TestContext = setup(&mut header);

        let wav = read_wav(&ctx.path).unwrap();

        teardown(ctx.path);
        assert_eq!(wav.sample_rate, 44100);
    }

    #[test]
    fn extract_data() {
        let mut header = Vec::new();
        write_wav_header(&mut header);
        write_junk_chunk(&mut header);
        write_fmt_chunk(&mut header);
        write_data_chunk(&mut header);
        let ctx: TestContext = setup(&mut header);

        let wav = read_wav(&ctx.path).unwrap();

        teardown(ctx.path);
        assert_eq!(wav.data, vec![1, 2, 3, 4]);
    }
    #[test]
    fn invalid_fmt_chunk() {
        let mut header = Vec::new();
        write_wav_header(&mut header);
        header.extend_from_slice(b"fmt ");
        header.extend_from_slice(&4u32.to_le_bytes());
        header.extend_from_slice(&[0u8; 4]);
        let ctx: TestContext = setup(&mut header);

        let res = read_wav(&ctx.path);

        teardown(ctx.path);
        assert!(res.is_err());
    }
    #[test]
    fn find_fmt_after_data() {
        let mut header = Vec::new();
        write_wav_header(&mut header);
        write_data_chunk(&mut header);
        header.extend_from_slice(b"fmt ");
        header.extend_from_slice(&16u32.to_le_bytes());
        header.extend_from_slice(&1u16.to_le_bytes());
        header.extend_from_slice(&2u16.to_le_bytes());
        header.extend_from_slice(&44_100u32.to_le_bytes());
        header.extend_from_slice(&[0u8; 4]);
        header.extend_from_slice(&[0u8; 2]);
        header.extend_from_slice(&16u16.to_le_bytes());
        let ctx: TestContext = setup(&mut header);

        let wav = read_wav(&ctx.path).unwrap();

        teardown(ctx.path);
        assert_eq!(wav.sample_rate, 44_100);
        assert_eq!(wav.channels, 2);
        assert_eq!(wav.bits_per_sample, 16);
        assert_eq!(wav.data, vec![1, 2, 3, 4]);
    }
    #[test]
    fn wav_without_fmt() {
        let mut header = Vec::new();
        write_wav_header(&mut header);
        write_data_chunk(&mut header);
        let ctx: TestContext = setup(&mut header);

        let res = read_wav(&ctx.path);

        teardown(ctx.path);
        assert!(res.is_err());
    }

    #[test]
    fn wav_without_data() {
        let mut header = Vec::new();
        write_wav_header(&mut header);
        write_fmt_chunk(&mut header);
        let ctx: TestContext = setup(&mut header);

        let res = read_wav(&ctx.path);

        teardown(ctx.path);
        assert!(res.is_err());
    }

    #[test]
    fn reject_non_pcm() {
        let mut header = Vec::new();
        write_wav_header(&mut header);
        header.extend_from_slice(b"fmt ");
        header.extend_from_slice(&16u32.to_le_bytes());

        // AudioFormat = 3 (IEEE float)
        header.extend_from_slice(&3u16.to_le_bytes());

        header.extend_from_slice(&1u16.to_le_bytes());
        header.extend_from_slice(&44_100u32.to_le_bytes());
        header.extend_from_slice(&88_200u32.to_le_bytes());
        header.extend_from_slice(&2u16.to_le_bytes());
        header.extend_from_slice(&16u16.to_le_bytes());

        write_data_chunk(&mut header);
        let ctx = setup(&mut header);

        let res = read_wav(&ctx.path);

        teardown(ctx.path);
        assert!(res.is_err());
    }

    #[test]
    fn reject_non_16bit_pcm() {
        let mut header = Vec::new();
        write_wav_header(&mut header);
        header.extend_from_slice(b"fmt ");
        header.extend_from_slice(&16u32.to_le_bytes());
        // PCM
        header.extend_from_slice(&1u16.to_le_bytes());
        // Mono
        header.extend_from_slice(&1u16.to_le_bytes());
        header.extend_from_slice(&44_100u32.to_le_bytes());
        header.extend_from_slice(&88_200u32.to_le_bytes());
        header.extend_from_slice(&2u16.to_le_bytes());
        // 24-bit
        header.extend_from_slice(&24u16.to_le_bytes());
        write_data_chunk(&mut header);
        let ctx = setup(&mut header);

        let res = read_wav(&ctx.path);

        teardown(ctx.path);
        assert!(res.is_err());
    }
    #[test]
    fn write_wav_creates_valid_header() {
        let wav = Wav {
            sample_rate: 44_100,
            channels: 1,
            bits_per_sample: 16,
            data: vec![1, 2, 3, 4],
        };

        let path = "output.wav";

        write_wav(&wav, path).unwrap();

        let mut file = File::open(path).unwrap();
        let mut header = [0u8; 12];
        file.read_exact(&mut header).unwrap();

        teardown(path.to_string());
        assert_eq!(&header[0..4], b"RIFF");
        assert_eq!(&header[8..12], b"WAVE");
    }
}
