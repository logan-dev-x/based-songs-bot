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
    use crate::test_helpers::*;

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
}
