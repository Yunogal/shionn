use std::fs::File;
use std::io::{self, BufWriter, Read, Seek, SeekFrom, Write};
use std::mem::{MaybeUninit, transmute};
use std::path::Path;
use std::slice;

use encoding_rs::SHIFT_JIS;

#[derive(Debug, Copy, Clone)]
pub struct Info {
    pub name: [u8; 32],
    pub size: u32,
    pub address: u32,
}
impl Info {
    pub fn name(&self) -> &str {
        let len = self.name.iter().position(|&c| c == 0).unwrap_or(32);
        str::from_utf8(&self.name[..len]).unwrap_or("")
    }
}

pub struct Text {
    name: [u8; 12],
    length: u32,
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

pub fn extract(file: &Path, base: &Path) -> io::Result<()> {
    let mut file = File::open(file)?;
    let mut buffer = [0u8; 4];
    file.read_exact(&mut buffer)?;
    if buffer != *b"PAC " {
        return Ok(());
    }
    file.seek(SeekFrom::Current(4))?;

    let mut count = [0u8; 4];
    file.read_exact(&mut count)?;
    let count: u32 = unsafe { transmute(count) };

    file.seek(SeekFrom::Start(0x804))?;

    let mut buffer: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(40 * count as usize);
    let raw_bytes =
        unsafe { slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, 40 * count as usize) };

    file.read_exact(raw_bytes)?;
    let info_ptr = Box::into_raw(buffer) as *mut Info;
    let info: Box<[Info]> =
        unsafe { Box::from_raw(slice::from_raw_parts_mut(info_ptr, count as usize)) };
    let max = info
        .iter()
        .map(|info| info.size)
        .max()
        .unwrap_or(4 * 1024 * 1024);
    let mut data: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(max as usize);

    for info in info.iter() {
        let data =
            unsafe { slice::from_raw_parts_mut(data.as_mut_ptr() as *mut u8, info.size as usize) };
        file.read_exact(data)?;
        if data[0] == b'$' {
            decode(&mut data[16..]);
            if data[..12] == *b"$TEXT_LIST__" {
                let json = File::create("$TEXT_LIST__.json")?;
                let mut writer = BufWriter::new(json);
                parse_data_to_json(data, &mut writer)?;
            }
        }
        let mut output_file = File::create(base.join(info.name()))?;
        output_file.write_all(data)?;
    }
    // let mut data = [0u8; 8];
    // file.read_exact(&mut data)?;
    // file.stream_position()?
    Ok(())
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

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;
    #[test]
    fn size() {
        assert_eq!(size_of::<Info>(), 40);
    }
}
