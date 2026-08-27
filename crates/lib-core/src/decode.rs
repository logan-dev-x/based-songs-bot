use crate::audio::Audio;
use crate::error::Error;
use minimp3::{Decoder, Error as Mp3Error};
use std::fs::File;
use std::path::Path;

pub fn decode<P: AsRef<Path>>(path: P) -> Result<Audio, Error> {
    let file = File::open(path)?;
    let mut decoder = Decoder::new(file);

    let mut samples = Vec::new();
    let mut sample_rate = 0;
    let mut channels = 0;

    loop {
        match decoder.next_frame() {
            Ok(frame) => {
                if sample_rate == 0 {
                    sample_rate = frame.sample_rate as u32;
                    channels = frame.channels as u8;
                }

                samples.extend(frame.data);
            }

            Err(Mp3Error::Eof) => break,

            Err(e) => {
                return Err(Error::Decode(format!("{:?}", e)));
            }
        }
    }

    if channels == 0 {
        return Err(Error::InvalidMp3);
    }

    Ok(Audio {
        samples,
        sample_rate,
        channels,
    })
}
