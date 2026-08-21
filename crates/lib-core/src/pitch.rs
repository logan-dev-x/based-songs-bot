pub fn pitch_ratio(from: f32, to: f32) -> f32 {
    to / from
}

pub fn resample(samples: &[i16], ratio: f32) -> Vec<i16> {
    let output_length = (samples.len() as f32 * ratio) as usize;
    let mut resampled = Vec::with_capacity(output_length);
    for index in 0..output_length {
        resampled.push(samples[(index as f32 / ratio) as usize]);
    }
    resampled
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        samples::{decode_samples, encode_samples},
        test_helpers::*,
    };

    #[test]
    fn pitch_ratio_test() {
        let ratio = pitch_ratio(440.0, 432.0);

        assert!((ratio - 0.981818).abs() < 0.000001);
    }

    #[test]
    fn resample_samples() {
        let samples = vec![0, 100, 200, 300];

        let result = resample(&samples, 0.5);

        assert_eq!(result, vec![0, 200]);
    }
    #[test]
    fn resample_half() {
        let samples = vec![0, 1, 2, 3, 4, 5, 6, 7];

        let result = resample(&samples, 0.5);

        assert_eq!(result.len(), 4);
    }

    #[test]
    fn resample_double() {
        let samples = vec![0, 1, 2, 3];

        let result = resample(&samples, 2.0);

        assert_eq!(result.len(), 8);
    }
    #[test]
    fn convert_samples_440_to_432() {
        let original = vec![0, 1000, 2000, 3000, 4000, 5000, 6000, 7000];

        let samples = decode_samples(&encode_samples(&original, 16).unwrap(), 16).unwrap();

        let ratio = pitch_ratio(440.0, 432.0);
        let resampled = resample(&samples, ratio);

        assert_eq!(resampled.len(), 7);
    }
}
