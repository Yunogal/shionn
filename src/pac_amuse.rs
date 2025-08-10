use std::arch::asm;
use std::borrow::Cow;
use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write};
use std::mem;
use std::path::Path;
use std::ptr;

use encoding_rs::SHIFT_JIS;
use memmap2::Mmap;

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
        unsafe { mem::transmute(&self.name[..len]) }
    }
}

pub fn decode(buffer: &mut [u8]) {
    let ptr = ptr::slice_from_raw_parts_mut(
        buffer.as_mut_ptr() as *mut u32,
        buffer.len() >> 2,
    );
    let buf = unsafe { &mut *ptr };
    let mut shift = 4 as i8;
    for i in buf {
        unsafe {
            asm!(
                "mov al, byte ptr [{ptr}]",
                "rol al, cl",
                "mov byte ptr [{ptr}], al",
                ptr = in(reg) i,
                in("cl") shift,
                out("al") _,
            );
        }
        *i ^= 0xF7D5859D; // 0xFF987DEE^0x084DF873
        shift = (shift + 1) & 7;
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

pub fn extract(mmap: Mmap, base: &Path) -> io::Result<()> {
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
