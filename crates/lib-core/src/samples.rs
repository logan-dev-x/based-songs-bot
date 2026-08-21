use std::io::{Error, ErrorKind, Result};

pub fn decode_samples(data: &Vec<u8>, bits_per_sample: u16) -> Result<Vec<i16>> {
    if bits_per_sample != 16 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Only 16-bit PCM is supported",
        ));
    }

    let mut samples = Vec::new();
    if data.len() % 2 != 0 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Invalid 16-bit PCM data",
        ));
    }
    for chunk in data.chunks_exact(2) {
        samples.push(i16::from_le_bytes([chunk[0], chunk[1]]));
    }

    Ok(samples)
}

pub fn encode_samples(samples: &Vec<i16>, bits_per_sample: u16) -> Result<Vec<u8>> {
    if bits_per_sample != 16 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Only 16-bit PCM is supported",
        ));
    }
    let mut data = Vec::with_capacity(samples.len() * 2);
    for sample in samples {
        data.extend_from_slice(&sample.to_le_bytes());
    }
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use crate::wav::read_wav;

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
    fn decode_stereo_samples() {
        let data = vec![0x64, 0x00, 0xF4, 0x01, 0xC8, 0x00, 0x58, 0x02];

        let samples = decode_samples(&data, 16).unwrap();

        assert_eq!(samples, vec![100, 500, 200, 600]);
    }

    #[test]
    fn encode_16bit_samples() {
        let samples = vec![0i16, 255i16, 256i16, -1i16];

        let data = encode_samples(&samples, 16).unwrap();

        assert_eq!(
            data,
            vec![
                0x00, 0x00, // 0
                0xFF, 0x00, // 255
                0x00, 0x01, // 256
                0xFF, 0xFF, // -1
            ]
        );
    }

    #[test]
    fn samples_round_trip() {
        let original = vec![0x00, 0x00, 0xFF, 0x00, 0x00, 0x01, 0xFF, 0xFF];

        let samples = decode_samples(&original, 16).unwrap();
        let encoded = encode_samples(&samples, 16).unwrap();

        assert_eq!(encoded, original);
    }

    #[test]
    fn decode_negative_samples() {
        let data = vec![
            0xFF, 0xFF, // -1
            0x18, 0xFC, // -1000
        ];

        let samples = decode_samples(&data, 16).unwrap();

        assert_eq!(samples, vec![-1, -1000]);
    }

    #[test]
    fn decode_16bit_odd_bytes() {
        let data = vec![0x00];

        let result = decode_samples(&data, 16);

        assert!(result.is_err());
    }
}
