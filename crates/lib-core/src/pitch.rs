use crate::{audio::Audio, error::Error};

pub fn pitch(audio: &Audio, from: f64, to: f64) -> Result<Audio, Error> {
    let frequency_ratio = to / from;
    let time_ratio = 1.0 / frequency_ratio;

    let channels = audio.channels as usize;
    let total_frames = audio.samples.len() / channels;

    let new_total_frames = (total_frames as f64 * time_ratio) as usize;

    let mut samples = Vec::with_capacity(new_total_frames * channels);

    for i in 0..new_total_frames {
        let src_pos = i as f64 / time_ratio;
        let index = src_pos.floor() as usize;
        let frac = src_pos - index as f64;

        for c in 0..channels {
            let pos1 = index * channels + c;
            let pos2 = (index + 1) * channels + c;

            let val1 = *audio.samples.get(pos1).unwrap_or(&0) as f64;

            let val2 = *audio.samples.get(pos2).unwrap_or(&0) as f64;

            let interpolated = val1 * (1.0 - frac) + val2 * frac;

            samples.push(interpolated as i16);
        }
    }

    Ok(Audio {
        samples,
        sample_rate: audio.sample_rate,
        channels: audio.channels,
    })
}
