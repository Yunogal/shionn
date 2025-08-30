#[repr(C)]
pub struct Gax {
    pub signature: u32, //'\0\0\0\x01'
    pub key: [u8; 16],
}
#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(size_of::<Gax>(), 20);
    assert_eq!(align_of::<Gax>(), 4)
}

pub fn parce(content: &mut [u8]) {
    let (key, data) = content[4..].split_at_mut(16);
    for block in data.chunks_mut(16) {
        for (v, byte) in block.iter_mut().enumerate() {
            *byte ^= key[v];
        }
        update(key, block[block.len() - 2]);
    }
}
fn update(key: &mut [u8], ch: u8) {
    let t = ch;
    let ch = ch & 7;
    match ch {
        | 0 => {
            key[0] += t;
            key[3] += t + 2;
            key[4] = key[2] + t + 11;
            key[8] = key[6] + 7;
        },
        | 1 => {
            key[2] = key[9] + key[10];
            key[6] = key[7] + key[15];
            key[8] += key[1];
            key[15] = key[5] + key[3];
        },
        | 2 => {
            key[1] += key[2];
            key[5] += key[6];
            key[7] += key[8];
            key[10] += key[11];
        },
        | 3 => {
            key[9] = key[2] + key[1];
            key[11] = key[6] + key[5];
            key[12] = key[8] + key[7];
            key[13] = key[11] + key[10];
        },
        | 4 => {
            key[0] = key[1] + 111;
            key[3] = key[4] + 71;
            key[4] = key[5] + 17;
            key[14] = key[15] + 64;
        },
        | 5 => {
            key[2] += key[10];
            key[4] = key[5] + key[12];
            key[6] = key[8] + key[14];
            key[8] = key[11] + key[0];
        },
        | 6 => {
            key[9] = key[11] + key[1];
            key[11] = key[13] + key[3];
            key[13] = key[15] + key[5];
            key[15] = key[9] + key[7];
            key[1] = key[9] + key[5];
            key[2] = key[10] + key[6];
            key[3] = key[11] + key[7];
            key[4] = key[12] + key[8];
        },
        | 7 => {
            key[1] = key[9] + key[5];
            key[2] = key[10] + key[6];
            key[3] = key[11] + key[7];
            key[4] = key[12] + key[8];
        },
        | _ => {},
    }
}
