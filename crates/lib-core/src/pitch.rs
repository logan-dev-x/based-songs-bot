pub fn pitch_ratio(from: f32, to: f32) -> f32 {
    to / from
}

pub fn resample(samples: &[i16], ratio: f32, channels: u16) -> Vec<i16> {
    let channels = channels as usize;
    let input_frames = samples.len() / channels;
    let output_frames = (input_frames as f32 * ratio) as usize;
    let output_length = output_frames * channels;

    let mut resampled = Vec::with_capacity(output_length);

    for frame_index in 0..output_frames {
        let source_frame = (frame_index as f32 / ratio) as usize;

        for channel in 0..channels {
            let source_index = source_frame * channels + channel;
            resampled.push(samples[source_index]);
        }
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

        let result = resample(&samples, 0.5, 1);

        assert_eq!(result, vec![0, 200]);
    }
    #[test]
    fn resample_half() {
        let samples = vec![0, 1, 2, 3, 4, 5, 6, 7];

        let result = resample(&samples, 0.5, 1);

        assert_eq!(result.len(), 4);
    }

    #[test]
    fn resample_double() {
        let samples = vec![0, 1, 2, 3];

        let result = resample(&samples, 2.0, 1);

        assert_eq!(result.len(), 8);
    }
    #[test]
    fn convert_samples_440_to_432() {
        let original = vec![0, 1000, 2000, 3000, 4000, 5000, 6000, 7000];

        let samples = decode_samples(&encode_samples(&original, 16).unwrap(), 16).unwrap();

        let ratio = pitch_ratio(440.0, 432.0);
        let resampled = resample(&samples, ratio, 1);

        assert_eq!(resampled.len(), 7);
    }
}
