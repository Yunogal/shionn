pub fn lz(input: &[u8], output: &mut [u8]) {
    let mut frame = [0; 0x1000];
    let mut frame_pos = 0xFEE;
    let mut src_pos = 0;
    let mut dst_pos = 0;
    let input_len = input.len();
    while src_pos < input_len && dst_pos < output.len() {
        let ctl = input[src_pos];
        src_pos += 1;

        let mut bit: u16 = 1;
        while bit != 0x100 && src_pos < input_len {
            if (ctl & bit as u8) != 0 {
                let b = input[src_pos];
                src_pos += 1;
                frame[frame_pos] = b;
                frame_pos = (frame_pos + 1) & (0x1000 - 1);

                output[dst_pos] = b;
                dst_pos += 1;
            } else {
                let lo = input[src_pos] as usize;
                let hi = input[src_pos + 1] as usize;
                src_pos += 2;
                let mut offset = ((hi & 0xF0) << 4) | lo; // 12-bit offset
                let mut count = 3 + (hi & 0x0F); // length

                while count > 0 && dst_pos < output.len() {
                    let v = frame[offset];
                    offset = (offset + 1) & (0x1000 - 1);

                    frame[frame_pos] = v;
                    frame_pos = (frame_pos + 1) & (0x1000 - 1);

                    output[dst_pos] = v;
                    dst_pos += 1;
                    count -= 1;
                }
            }
            bit <<= 1;
        }
    }
}
