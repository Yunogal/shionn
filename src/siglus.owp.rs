pub fn parse(content: &mut [u8]) {
    for i in content {
        *i ^= 0x39;
    }
}
