#[derive(Debug, Clone)]
pub struct Audio {
    pub samples: Vec<i16>,
    pub sample_rate: u32,
    pub channels: u8,
}
