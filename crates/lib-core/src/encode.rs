use crate::audio::Audio;
use crate::error::Error;
use mp3lame_encoder::{Bitrate, Builder, Encoder, FlushNoGap, InterleavedPcm, MonoPcm};
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn create_encoder(channels: u8, sample_rate: u32) -> Result<Encoder, Error> {
    let mut builder =
        Builder::new().ok_or_else(|| Error::Encode("Falha ao criar LAME builder".into()))?;

    builder
        .set_num_channels(channels)
        .map_err(|e| Error::Encode(format!("{:?}", e)))?;

    builder
        .set_sample_rate(sample_rate)
        .map_err(|e| Error::Encode(format!("{:?}", e)))?;

    builder
        .set_brate(Bitrate::Kbps320)
        .map_err(|e| Error::Encode(format!("{:?}", e)))?;

    builder
        .set_quality(mp3lame_encoder::Quality::Best)
        .map_err(|e| Error::Encode(format!("{:?}", e)))?;

    builder
        .build()
        .map_err(|e| Error::Encode(format!("{:?}", e)))
}

fn encode_samples(encoder: &mut Encoder, audio: &Audio) -> Result<Vec<u8>, Error> {
    let mut output = Vec::new();

    let chunk_size = 8192 * audio.channels as usize;

    for chunk in audio.samples.chunks(chunk_size) {
        if audio.channels == 1 {
            encoder
                .encode_to_vec(MonoPcm(chunk), &mut output)
                .map_err(|e| Error::Encode(format!("{:?}", e)))?;
        } else {
            encoder
                .encode_to_vec(InterleavedPcm(chunk), &mut output)
                .map_err(|e| Error::Encode(format!("{:?}", e)))?;
        }
    }

    encoder
        .flush_to_vec::<FlushNoGap>(&mut output)
        .map_err(|e| Error::Encode(format!("{:?}", e)))?;

    Ok(output)
}

pub fn encode<P: AsRef<Path>>(audio: &Audio, path: P) -> Result<(), Error> {
    let mut encoder = create_encoder(audio.channels, audio.sample_rate)?;

    let buf = encode_samples(&mut encoder, audio)?;

    let mut output = File::create(path)?;

    output.write_all(&buf)?;

    Ok(())
}
