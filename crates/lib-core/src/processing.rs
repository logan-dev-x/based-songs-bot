use std::io::Result;

use crate::{
    pitch::{pitch_ratio, resample},
    samples::{decode_samples, encode_samples},
    wav::Wav,
};

pub fn change_pitch(wav: &Wav, from: f32, to: f32) -> Result<Wav> {
    let samples = decode_samples(&wav.data, wav.bits_per_sample)?;
    let ratio = pitch_ratio(from, to);
    let resampled = resample(&samples, ratio);
    let data = encode_samples(&resampled, wav.bits_per_sample)?;
    Ok(Wav {
        sample_rate: wav.sample_rate,
        channels: wav.channels,
        bits_per_sample: wav.bits_per_sample,
        data,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn change_wav_pitch() {
        let wav = Wav {
            sample_rate: 44_100,
            channels: 1,
            bits_per_sample: 16,
            data: encode_samples(&vec![0, 1000, 2000, 3000], 16).unwrap(),
        };

        let result = change_pitch(&wav, 440.0, 432.0).unwrap();

        assert_eq!(result.sample_rate, 44_100);
        assert_eq!(result.channels, 1);
        assert_eq!(result.bits_per_sample, 16);
        assert!(result.data.len() < wav.data.len());
    }
}
