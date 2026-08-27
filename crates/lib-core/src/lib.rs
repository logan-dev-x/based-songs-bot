use minimp3::{Decoder, Error as Mp3Error};
use mp3lame_encoder::{Builder, Encoder, FlushNoGap, InterleavedPcm, MonoPcm};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub mod error;

/// Lê o arquivo MP3 e retorna: (Amostras de Áudio, Sample Rate, Quantidade de Canais)
fn decode<P: AsRef<Path> + std::fmt::Display>(path: P) -> (Vec<i16>, u32, u8) {
    let err_msg = &format_args!("Erro: Arquivo {} não encontrado", path).to_string();
    let mut decoder = Decoder::new(File::open(path).expect(err_msg));
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
            Err(e) => panic!("Erro ao decodificar: {:?}", e),
        }
    }
    (samples, sample_rate, channels)
}

/// Altera a frequência do áudio (ex: 440.0 para 432.0)
/// Passar &[i16] (Slice) é mais idiomático no Rust do que &Vec<i16>
fn pitch(from: f64, to: f64, samples: &[i16], channels: u8) -> Vec<i16> {
    let ratio = from / to;

    let channels_usize = channels as usize;
    let total_frames = samples.len() / channels_usize;
    let new_total_frames = (total_frames as f64 * ratio) as usize;

    let mut output_samples = Vec::with_capacity(new_total_frames * channels_usize);

    for i in 0..new_total_frames {
        let src_pos = i as f64 / ratio;
        let index = src_pos.floor() as usize;
        let frac = src_pos - index as f64;

        for c in 0..channels_usize {
            let pos1 = index * channels_usize + c;
            let pos2 = (index + 1) * channels_usize + c;

            let val1 = samples.get(pos1).copied().unwrap_or(0) as f64;
            let val2 = samples.get(pos2).copied().unwrap_or(0) as f64;

            let interpolated = val1 * (1.0 - frac) + val2 * frac;

            output_samples.push(interpolated as i16);
        }
    }

    output_samples
}

/// Cria o Encoder
fn create_encoder(channels: u8, sample_rate: u32) -> Encoder {
    let mut mp3_builder = Builder::new().expect("Erro ao criar LAME builder");
    mp3_builder
        .set_num_channels(channels)
        .expect("Erro ao configurar canais");
    mp3_builder
        .set_sample_rate(sample_rate)
        .expect("Erro ao configurar sample rate");
    mp3_builder
        .set_brate(mp3lame_encoder::Bitrate::Kbps320)
        .expect("Erro ao configurar bitrate");
    mp3_builder
        .set_quality(mp3lame_encoder::Quality::Best)
        .expect("Erro ao configurar qualidade");

    mp3_builder
        .build()
        .expect("Erro ao inicializar LAME encoder")
}

/// O Encoder é recebido via '&mut Encoder' para não ser destruído após o uso.
fn encode(encoder: &mut Encoder, channels: u8, output_samples: &[i16]) -> Vec<u8> {
    let mut output = Vec::new();

    let frames_per_chunk = 1152;
    let chunk_size = frames_per_chunk * channels as usize;

    for chunk in output_samples.chunks(chunk_size) {
        if chunk.len() < channels as usize {
            break;
        }

        let frames = chunk.len() / channels as usize;

        // LAME pode precisar de até aproximadamente 1.25 bytes
        // por amostra, além de espaço adicional interno.
        let required = 7200 + (frames * 5 / 4);

        output.reserve(required);

        if channels == 1 {
            encoder
                .encode_to_vec(MonoPcm(chunk), &mut output)
                .expect("Erro ao encodar MP3");
        } else {
            encoder
                .encode_to_vec(InterleavedPcm(chunk), &mut output)
                .expect("Erro ao encodar MP3");
        }
    }

    // O LAME recomenda pelo menos 7200 bytes para o flush.
    output.reserve(7200);

    encoder
        .flush_to_vec::<FlushNoGap>(&mut output)
        .expect("Erro no flush do encoder");

    output
}

/// Salva o arquivo junto com as tags do LAME.
fn export<P: AsRef<Path>>(output_path: P, encoder: &mut Encoder, buf: &[u8]) {
    let mut output = File::create(output_path).expect("Erro ao criar arquivo de saída");

    if encoder.lame_tag_size() > 0 {
        let id3_size = encoder.id3v2_tag_size();
        let mut lame_tag = Vec::new();
        lame_tag.reserve(encoder.lame_tag_size());

        encoder
            .lame_tag_encode_to_vec(&mut lame_tag)
            .expect("Erro ao gerar lame tag");

        output.write_all(&buf[..id3_size]).unwrap();
        output.write_all(&lame_tag).unwrap();
        output.write_all(&buf[id3_size..]).unwrap();
    } else {
        output.write_all(buf).unwrap();
    }
}

/// Função principal que atua como API Pública da biblioteca
pub fn pitch_shift<P: AsRef<Path> + std::fmt::Display>(
    input_path: P,
    output_path: P,
    from: f64,
    to: f64,
) {
    println!("Lendo e decodificando arquivo...");
    let (samples, sample_rate, channels) = decode(input_path);

    println!("Aplicando pitch-shift de {}Hz para {}Hz...", from, to);
    let converted_samples = pitch(from, to, &samples, channels);

    println!("Codificando o novo áudio...");
    println!("Sample rate: {}", sample_rate);
    println!("Channels: {}", channels);
    println!("Samples: {}", samples.len());
    println!("Converted samples: {}", converted_samples.len());

    assert!(channels == 1 || channels == 2);
    assert!(converted_samples.len() % channels as usize == 0);
    let mut encoder = create_encoder(channels, sample_rate);

    // Passamos as amostras já CONVERTIDAS, e emprestamos (&mut) o encoder
    let buf = encode(&mut encoder, channels, &converted_samples);

    println!("Salvando em disco...");
    export(output_path, &mut encoder, &buf);

    println!("Concluído com sucesso!");
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn converts_440hz_to_432hz() {
        let sample_rate = 48_000.0;
        let frequency = 440.0;
        let duration = 1.0;

        let samples_count = (sample_rate * duration) as usize;

        let input: Vec<i16> = (0..samples_count)
            .map(|i| {
                let t = i as f64 / sample_rate;
                let value = (2.0 * std::f64::consts::PI * frequency * t).sin();

                (value * i16::MAX as f64) as i16
            })
            .collect();

        let output = pitch(440.0, 432.0, &input, 1);

        let measured = estimate_frequency(&output, sample_rate);

        assert!(
            (measured - 432.0).abs() < 1.0,
            "esperado ~432 Hz, obtido {} Hz",
            measured
        );
    }
}
