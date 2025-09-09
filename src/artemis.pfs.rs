use std::fs::{File, create_dir_all};
use std::io::{Result, Write};
use std::mem::transmute;
use std::path::Path;
use std::ptr;

use crate::shionn_stream::ByteStream;

#[repr(C, packed)]
pub struct PF {
    pub signature: [u8; 3], //'pf8'
    pub len: u32,
    pub count: u32,
}

// pub struct Entry {
//     pub name_len: u32,
//     pub name: [u8; name_len],
//     pub zero: u32,
//     pub address: u32,
//     pub size: u32,
// }

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<PF>(), 1);
    assert_eq!(size_of::<PF>(), 11);
}

pub fn extract(content: &mut [u8], base: &Path) -> Result<()> {
    let ptr = content.as_ptr();
    let PF {
        signature: _,
        len,
        count,
    } = unsafe { ptr.cast::<PF>().read_unaligned() };

    let header = unsafe { &*ptr::slice_from_raw_parts(ptr.add(7), len as usize) };
    let key = sha1(header);
    let header =
        unsafe { &*ptr::slice_from_raw_parts(ptr.add(11), len as usize - 4) };

    let mut stream = ByteStream::new(header);
    for _ in 0..count {
        let name_len: u32 = stream.read();
        let bytes = stream.get(name_len as usize);
        let name: &str = unsafe { transmute(bytes) };
        stream.skip(4);
        let address: u32 = stream.read();
        let address = address as usize;
        let size: u32 = stream.read();
        let size = size as usize;
        let data = &mut content[address..address + size];

        for (i, byte) in data.iter_mut().enumerate() {
            *byte ^= key[i % 20]
        }
        let file_path = base.join(name);
        if let Some(parent) = file_path.parent() {
            create_dir_all(parent)?;
        }
        let mut file = File::create(file_path)?;
        file.write_all(&data)?;
    }
    Ok(())
}

#[inline(always)]
pub const fn left_rotate(value: u32, bits: u32) -> u32 {
    (value << bits) | (value >> (32 - bits))
}

pub fn sha1(input: &[u8]) -> [u8; 20] {
    let mut bytes = input.to_vec();

    let bit_len = (bytes.len() as u64) * 8;

    bytes.push(0x80);

    while (bytes.len() % 64) != 56 {
        bytes.push(0);
    }

    bytes.extend_from_slice(&bit_len.to_be_bytes());

    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xEFCDAB89;
    let mut h2: u32 = 0x98BADCFE;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xC3D2E1F0;

    for chunk in bytes.chunks(64) {
        let mut w = [0u32; 80];

        for i in 0..16 {
            w[i] = ((chunk[4 * i] as u32) << 24)
                | ((chunk[4 * i + 1] as u32) << 16)
                | ((chunk[4 * i + 2] as u32) << 8)
                | (chunk[4 * i + 3] as u32);
        }

        for i in 16..80 {
            w[i] = left_rotate(w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16], 1);
        }

        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        for i in 0..80 {
            let (f, k) = match i {
                | 0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                | 20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                | 40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                | _ => (b ^ c ^ d, 0xCA62C1D6),
            };

            let temp = left_rotate(a, 5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);
            e = d;
            d = c;
            c = left_rotate(b, 30);
            b = a;
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    u32_array_to_u8_array([h0, h1, h2, h3, h4])
}
fn u32_array_to_u8_array(hash_u32: [u32; 5]) -> [u8; 20] {
    let mut bytes = [0u8; 20];
    for (i, word) in hash_u32.iter().enumerate() {
        let b = word.to_be_bytes();
        bytes[i * 4..i * 4 + 4].copy_from_slice(&b);
    }
    bytes
}
