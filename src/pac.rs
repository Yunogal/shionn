use std::borrow::Cow;
use std::fs::OpenOptions;
use std::io::{self, BufWriter, Error, ErrorKind, Write};
use std::path::Path;
use std::ptr;

use encoding_rs::SHIFT_JIS;
use memmap2::MmapOptions;

const OFFSET: usize = 0x804;

#[derive(Debug, Clone)]
pub struct Info {
    pub name: [u8; 32],
    pub size: u32,
    pub address: u32,
}

impl Info {
    pub fn name(&self) -> &str {
        let mut len: usize = 0;
        for i in self.name {
            if i != 0x00 {
                len += 1;
            }
        }

        #[cfg(debug_assertions)]
        let data = str::from_utf8(&self.name[..len]).unwrap_or("");
        #[cfg(not(debug_assertions))]
        let data = unsafe { str::from_utf8_unchecked(&self.name[..len]) };

        data
    }
}

pub fn decode(buffer: &mut [u8]) {
    let len = buffer.len();

    for (index, i) in (0..len - len % 4).enumerate() {
        buffer[i] = match index % 32 {
            | 0 => {
                buffer[i] ^= 0xd9;
                (buffer[i] << 4) | (buffer[i] >> 4)
            },
            | 4 => {
                buffer[i] ^= 0xEC;
                (buffer[i] << 5) | (buffer[i] >> 3)
            },
            | 8 => {
                buffer[i] ^= 0x76;
                (buffer[i] << 6) | (buffer[i] >> 2)
            },
            | 12 => {
                buffer[i] ^= 0x3B;
                (buffer[i] << 7) | (buffer[i] >> 1)
            },
            | 16 => buffer[i] ^ 0x9d,
            | 20 => {
                buffer[i] ^= 0xCE;
                (buffer[i] << 1) | (buffer[i] >> 7)
            },
            | 24 => {
                buffer[i] ^= 0x67;
                (buffer[i] << 2) | (buffer[i] >> 6)
            },
            | 28 => {
                buffer[i] ^= 0xB3;
                (buffer[i] << 3) | (buffer[i] >> 5)
            },
            | 1 | 5 | 9 | 13 | 17 | 21 | 25 | 29 => buffer[i] ^ 0x85,
            | 2 | 6 | 10 | 14 | 18 | 22 | 26 | 30 => buffer[i] ^ 0xD5,
            | 3 | 7 | 11 | 15 | 19 | 23 | 27 | 31 => buffer[i] ^ 0xF7,
            | _ => buffer[i],
        };
    }
}

pub fn parse_data_to_json<W: Write>(input: &[u8], mut out: W) -> io::Result<()> {
    #[cfg(debug_assertions)]
    {
        if input.len() < 16 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "input too short",
            ));
        }
    }
    let name = String::from_utf8_lossy(&input[..12]);
    let length = u32::from_le_bytes(input[12..16].try_into().unwrap());
    let mut pos = 16;

    write!(out, "{{\n")?;
    write!(out, "  \"name\": \"{name}\",\n")?;
    write!(out, "  \"length\": {length:}")?;

    for i in 0..length {
        #[cfg(debug_assertions)]
        debug_assert_eq!(&input[pos..pos + 4], i.to_le_bytes());
        pos += 4;
        let start = pos;
        while input[pos] != 0 {
            pos += 1;
        }

        let str_bytes = &input[start..pos];
        pos += 1;

        let (cow, ..) = SHIFT_JIS.decode(str_bytes);
        write!(out, ",\n  \"0x{i:08X}\": \"{cow}\"")?;
    }
    write!(out, "\n}}")?;
    out.flush()?;
    Ok(())
}

pub fn extract(file: &Path, base: &Path) -> io::Result<()> {
    let file = OpenOptions::new().read(true).open(file)?;

    let mmap = unsafe { MmapOptions::new().map(&file)? };

    if mmap[0..4] != *b"PAC " {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Invalid signature: expected signature 'PAC\x20' ",
        ));
    }
    let count = &mmap[8..12];
    let count = unsafe {
        let ptr = count.as_ptr() as *const u32;
        *ptr
    } as usize;

    let end = OFFSET + 40 * count;
    let entry = &mmap[OFFSET..end];
    let entry = entry.as_ptr() as *const Info;
    let entry = unsafe { &*ptr::slice_from_raw_parts(entry, count) };

    for i in 0..count {
        let size = entry[i].size as usize;
        let address = entry[i].address as usize;
        let name = entry[i].name();
        let content = &mmap[address..address + size];
        let mut content = Cow::Borrowed(content);
        if content[0] == b'$' {
            decode(&mut content.to_mut()[16..]);
            if content[..12] == *b"$TEXT_LIST__" {
                let json = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open("$TEXT_LIST__.json")?;
                let mut writer = BufWriter::new(json);
                parse_data_to_json(content.as_ref(), &mut writer)?;
            }
        }
        let mut extract_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(base.join(name))?;
        extract_file.write_all(content.as_ref())?;
    }
    Ok(())
}
