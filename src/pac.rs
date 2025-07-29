#![allow(unnecessary_transmutes)]
#![allow(unused_must_use)]

use encoding_rs::SHIFT_JIS;
use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::mem;
use std::path::Path;
use std::slice;
#[derive(Debug, Copy, Clone)]
pub struct Info {
    pub name: [u8; 32],
    pub size: u32,
    #[allow(dead_code)]
    pub address: u32,
}
impl Info {
    pub fn name(&self) -> &str {
        let len = self.name.iter().position(|&c| c == 0).unwrap_or(32);
        std::str::from_utf8(&self.name[..len]).unwrap_or("")
    }
}

#[allow(dead_code)]
pub struct Text {
    name: [u8; 12],
    length: u32,
}

pub fn decode(buffer: &mut [u8]) {
    let mut global_index = 0;
    let len = buffer.len();

    for i in 0..len - len % 4 {
        let processed = match global_index % 32 {
            0 => {
                buffer[i] ^= 0xd9;
                (buffer[i] << 4) | (buffer[i] >> 4)
            }
            4 => {
                buffer[i] ^= 0xEC;
                (buffer[i] << 5) | (buffer[i] >> 3)
            }
            8 => {
                buffer[i] ^= 0x76;
                (buffer[i] << 6) | (buffer[i] >> 2)
            }
            12 => {
                buffer[i] ^= 0x3B;
                (buffer[i] << 7) | (buffer[i] >> 1)
            }
            16 => buffer[i] ^ 0x9d,
            20 => {
                buffer[i] ^= 0xCE;
                (buffer[i] << 1) | (buffer[i] >> 7)
            }
            24 => {
                buffer[i] ^= 0x67;
                (buffer[i] << 2) | (buffer[i] >> 6)
            }
            28 => {
                buffer[i] ^= 0xB3;
                (buffer[i] << 3) | (buffer[i] >> 5)
            }
            1 | 5 | 9 | 13 | 17 | 21 | 25 | 29 => buffer[i] ^ 0x85,
            2 | 6 | 10 | 14 | 18 | 22 | 26 | 30 => buffer[i] ^ 0xD5,
            3 | 7 | 11 | 15 | 19 | 23 | 27 | 31 => buffer[i] ^ 0xF7,
            _ => buffer[i],
        };
        buffer[i] = processed;
        global_index += 1;
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
    let count: u32 = unsafe { mem::transmute(count) };

    file.seek(SeekFrom::Start(0x804))?;

    let mut buffer: Box<[mem::MaybeUninit<u8>]> = Box::new_uninit_slice(40 * count as usize);
    let raw_bytes =
        unsafe { slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, 40 * count as usize) };

    file.read_exact(raw_bytes)?;
    let info_ptr = Box::into_raw(buffer) as *mut Info;
    let info: Box<[Info]> =
        unsafe { Box::from_raw(slice::from_raw_parts_mut(info_ptr, count as usize)) };

    fs::create_dir(base)?;
    let max = info
        .iter()
        .map(|info| info.size)
        .max()
        .unwrap_or(4 * 1024 * 1024);
    let mut data: Box<[mem::MaybeUninit<u8>]> = Box::new_uninit_slice(max as usize);

    for info in info.iter() {
        let buffer_u8 = unsafe {
            std::slice::from_raw_parts_mut((&mut *data).as_mut_ptr() as *mut u8, info.size as usize)
        };
        file.read_exact(buffer_u8)?;
        if buffer_u8[0] == b'$' {
            decode(&mut buffer_u8[16..]);
            // if buffer_u8[..12] == *b"$TEXT_LIST__" {
            //     let mut json = File::create("text.json")?;
            //     parse_data_to_json(&buffer_u8, &mut json)?;
            // }
        }
        let mut output_file = File::create(base.join(info.name()))?;
        output_file.write_all(buffer_u8)?;
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

    writeln!(out, "{{")?;
    writeln!(out, "  \"name\": \"{}\",", escape_json_string(&name))?;
    writeln!(out, "  \"length\": {:},", length)?;

    for i in 0..length {
        debug_assert_eq!(&input[pos..pos + 4], &i.to_le_bytes());
        pos += 4;
        let start = pos;
        while pos < input.len() && input[pos] != 0 {
            pos += 1;
        }

        let str_bytes = &input[start..pos];
        pos += 1;

        let (cow, _, _) = SHIFT_JIS.decode(str_bytes);

        let key = format!("0x{:08X}", i);
        let value = escape_json_string(&cow);

        write!(out, "    \"{}\": \"{}\"", key, value);
        if i + 1 != length {
            writeln!(out, ",");
        } else {
            writeln!(out);
        }
    }

    writeln!(out, "}}");
    Ok(())
}

fn escape_json_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
