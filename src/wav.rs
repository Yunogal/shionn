pub struct WAV {
    pub id: [u8; 4], // 'RIFF'
    pub size: u32,
    pub format: [u8; 4], // 'WAVE'
    pub id2: [u8; 4],    // 'fmt '
    pub size2: u32,
    pub audio_format: u16,
    pub channals: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    //
    pub id3: [u8; 4], // 'data'
    pub size3: u32,
}
// pub struct Chunk {
//     pub id: [u8; 4],
//     pub data_size: u32,
//     pub data: [u8; data_size],
// }
