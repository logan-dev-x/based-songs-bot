pub mod audio;
pub mod decode;
pub mod encode;
pub mod error;
pub mod pitch;

#[cfg(test)]
mod test {
    use crate::audio::Audio;
    use crate::pitch::pitch;

    fn estimate_frequency(samples: &[i16], sample_rate: f64) -> f64 {
        let mut crossings = 0;

        for i in 1..samples.len() {
            if samples[i - 1] < 0 && samples[i] >= 0 {
                crossings += 1;
            }
        }

        crossings as f64 * sample_rate / samples.len() as f64
    }

    #[test]
    fn pitch_440_to_432() {
        let sample_rate = 48_000;
        let frequency = 440.0;

        let input: Vec<i16> = (0..sample_rate)
            .map(|n| {
                let t = n as f64 / sample_rate as f64;
                let sample = (2.0 * std::f64::consts::PI * frequency * t).sin();

                (sample * i16::MAX as f64 * 0.5) as i16
            })
            .collect();

        let audio = Audio {
            samples: input,
            sample_rate,
            channels: 1,
        };

        let input_frequency = estimate_frequency(&audio.samples, sample_rate as f64);

        let output = pitch(&audio, 440.0, 432.0).expect("pitch failed");

        let measured = estimate_frequency(&output.samples, sample_rate as f64);

        assert!(
            (input_frequency - 440.0).abs() < 2.0,
            "input should be approximately 440Hz, got {}Hz",
            input_frequency
        );

        assert!(
            (measured - 432.0).abs() < 2.0,
            "output should be approximately 432Hz, got {}Hz",
            measured
        );
    }
}
