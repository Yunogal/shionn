use std::borrow::Cow;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Error, ErrorKind, Write};
use std::mem::transmute;
use std::path::Path;

use memmap2::MmapOptions;

pub fn extract(file: &Path, base: &Path) -> io::Result<()> {
    let file = OpenOptions::new().read(true).open(file)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    if mmap[0..3] != *b"pf8" {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Invalid signature: expected signature 'pf8' ",
        ));
    }
    let length = bytes_to_u32_be(&mmap[3..7]) as usize;
    let end = 7 + length;
    let header = &mmap[7..end];
    let key = sha1(header);
    let count = bytes_to_u32_be(&header[..4]);
    let mut i = 1;
    let mut pos = 4;
    loop {
        if i > count {
            break;
        }
        let name_len = bytes_to_u32_be(&header[pos..pos + 4]) as usize;
        pos += 4;
        let bytes = &header[pos..pos + name_len];
        let name: &str = unsafe { transmute(bytes) };
        pos += name_len + 4;
        let address = bytes_to_u32_be(&header[pos..pos + 4]) as usize;
        pos += 4;
        let size = bytes_to_u32_be(&header[pos..pos + 4]) as usize;
        pos += 4;

        let data = &mmap[address..address + size];
        let mut content = Cow::Borrowed(data);
        for (i, byte) in content.to_mut().iter_mut().enumerate() {
            *byte ^= key[i % 20]
        }
        let file_path = base.join(name);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        #[inline]
        fn open_file_with_dir_create(path: &Path) -> io::Result<fs::File> {
            match OpenOptions::new()
                .create(true)
                .write(true)
                .open(path)
            {
                | Ok(file) => Ok(file),
                | Err(e) if e.kind() == ErrorKind::NotFound => {
                    if let Some(parent) = path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    OpenOptions::new()
                        .create(true)
                        .write(true)
                        .open(path)
                },
                | Err(e) => Err(e),
            }
        }
        let mut output_file = open_file_with_dir_create(&file_path)?;
        output_file.write_all(&content)?;
        i += 1;
    }
    Ok(())
}
#[inline(always)]
pub const fn bytes_to_u32_be(bytes: &[u8]) -> u32 {
    (bytes[0] as u32)
        | ((bytes[1] as u32) << 8)
        | ((bytes[2] as u32) << 16)
        | ((bytes[3] as u32) << 24)
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
