pub fn parse(content: &[u8]) -> &[u8] {
    let offset = content[0x1E] as usize;
    &content[0x20 + offset..]
}
