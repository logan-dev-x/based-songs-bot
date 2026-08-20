use std::fs::File;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result, prelude::*};
use std::path::Path;

struct Wav {
    sample_rate: u32,
    channels: u16,
    bits_per_sample: u16,
    data: Vec<u8>,
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
                return Err(Error::new(InvalidData, "Invalid fmt chunk"));
            }

            if u16::from_le_bytes(chunk.data[0..2].try_into().unwrap()) != 1 {
                return Err(Error::new(InvalidData, "Only PCM WAV is supported"));
            }

            bits_per_sample = Some(u16::from_le_bytes(chunk.data[14..16].try_into().unwrap()));
            if bits_per_sample.unwrap() != 16 {
                return Err(Error::new(InvalidData, "Only 16-bit is supported"));
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

fn decode_samples(data: &Vec<u8>, bits_per_sample: u16) -> Result<Vec<i16>> {
    if bits_per_sample != 16 {
        return Err(Error::new(InvalidData, "Only 16-bit PCM is supported"));
    }

    let mut samples = Vec::new();
    for chunk in data.chunks_exact(2) {
        samples.push(i16::from_le_bytes([chunk[0], chunk[1]]));
    }

    Ok(samples)
}

#[cfg(test)]
mod tests {
    use std::fs::{OpenOptions, remove_file};

    use super::*;

    fn write_wav_header(header: &mut Vec<u8>) {
        header.extend_from_slice(
            &[
                [b'R', b'I', b'F', b'F'].as_slice(),
                &[0u8; 4],
                [b'W', b'A', b'V', b'E'].as_slice(),
            ]
            .concat(),
        );
    }

    fn write_data_chunk(header: &mut Vec<u8>) {
        header.extend_from_slice(
            &[
                [b'd', b'a', b't', b'a'].as_slice(),
                &4u32.to_le_bytes(),
                &[1, 2, 3, 4],
            ]
            .concat(),
        );
    }

    fn write_junk_chunk(header: &mut Vec<u8>) {
        header.extend_from_slice(
            &[
                [b'J', b'U', b'N', b'K'].as_slice(),
                &4u32.to_le_bytes(),
                &[1, 2, 3, 4],
            ]
            .concat(),
        );
    }

    fn write_fmt_chunk(header: &mut Vec<u8>) {
        header.extend_from_slice(
            &[
                [b'f', b'm', b't', b' '].as_slice(),
                &16u32.to_le_bytes(),
                &1u16.to_le_bytes(),
                &[0u8; 12],
                &16u16.to_le_bytes(),
            ]
            .concat(),
        );
    }

    struct TestContext {
        path: String,
        file: File,
    }

    fn setup(header: &mut Vec<u8>) -> TestContext {
        let path = format!("{}.wav", rand::random_range(0..1000));
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .unwrap();
        file.write_all(header).unwrap();
        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        TestContext { path, file }
    }

    fn teardown(path: String) {
        remove_file(path).unwrap();
    }

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
    fn decode_16bit_samples() {
        let data = vec![0x00, 0x00, 0xFF, 0x00, 0x00, 0x01];

        let samples = decode_samples(&data, 16).unwrap();

        assert_eq!(samples, vec![0, 255, 256]);
    }

    #[test]
    fn decode_wav_data() {
        let mut header = Vec::new();
        write_wav_header(&mut header);
        header.extend_from_slice(b"fmt ");
        header.extend_from_slice(&16u32.to_le_bytes());
        header.extend_from_slice(&1u16.to_le_bytes());
        header.extend_from_slice(&1u16.to_le_bytes());
        header.extend_from_slice(&44_100u32.to_le_bytes());
        header.extend_from_slice(&88_200u32.to_le_bytes());
        header.extend_from_slice(&2u16.to_le_bytes());
        header.extend_from_slice(&16u16.to_le_bytes());

        header.extend_from_slice(b"data");
        header.extend_from_slice(&6u32.to_le_bytes());
        header.extend_from_slice(&[0x00, 0x00, 0xFF, 0x00, 0x00, 0x01]);
        let ctx: TestContext = setup(&mut header);
        let wav = read_wav(&ctx.path).unwrap();

        let samples = decode_samples(&wav.data, wav.bits_per_sample).unwrap();

        teardown(ctx.path);
        assert_eq!(samples, vec![0, 255, 256]);
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
    fn decode_stereo_samples() {
        let data = vec![0x64, 0x00, 0xF4, 0x01, 0xC8, 0x00, 0x58, 0x02];

        let samples = decode_samples(&data, 16).unwrap();

        assert_eq!(samples, vec![100, 500, 200, 600]);
    }
}
