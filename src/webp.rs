// https://developers.google.com/speed/webp/
// https://developers.google.com/speed/webp/docs/riff_container
// https://www.rfc-editor.org/rfc/rfc9649.html#name-riff-header
pub struct Webp {
    pub signature: [u8; 4], // 'RIFF'  // // Resource Interchange File Format
    pub size: u32,
    pub signature2: [u8; 4],   // 'WEBP'
    pub chunk_header: [u8; 4], //'VP8 ' or 'VP8L' or 'VP8X' or others
}

fn main() {}
