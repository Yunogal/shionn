use std::arch::asm;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::mem;
use std::path::Path;
use std::ptr;

use encoding_rs::SHIFT_JIS;

const OFFSET: usize = 0x804;

#[derive(Debug)]
#[repr(C)]
pub struct Pac {
    signature: u32,
    zero: u32,
    count: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct Entry {
    pub name: [u8; 32],
    pub size: u32,
    pub address: u32,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Pac>(), 4);
    assert_eq!(size_of::<Pac>(), 12);
    assert_eq!(align_of::<Entry>(), 4);
    assert_eq!(size_of::<Entry>(), 40);
}

impl Entry {
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
    let mut shift: i8 = 4;
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
    let length: u32 = unsafe {
        let ptr = input.as_ptr().add(12) as *const u32;
        ptr.read_unaligned()
    };

    let mut pos = 16;

    write!(
        out,
        "{{\n  \"name\": \"$TEXT_LIST__\",\n  \"length\": {length:}"
    )?;

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

pub fn extract(content: &mut [u8], base: &Path) -> io::Result<()> {
    let ptr: *const Pac = content.as_ptr().cast();
    let pac = unsafe { &*ptr };
    let count = pac.count as usize;
    let end = OFFSET + 40 * count;
    let entry = &content[OFFSET..end];
    let entry = entry.as_ptr() as *const Entry;
    let entry = unsafe { &*ptr::slice_from_raw_parts(entry, count) };

    for i in 0..count {
        let size = entry[i].size as usize;
        let address = entry[i].address as usize;
        let name = entry[i].name();
        let data = &mut content[address..address + size];
        if data[0] == b'$' {
            decode(&mut data[16..]);
            if data[..12] == *b"$TEXT_LIST__" {
                let json = File::create("$TEXT_LIST__.json")?;

                let mut writer = BufWriter::new(json);
                parse_data_to_json(data.as_ref(), &mut writer)?;
            }
        }
        let mut extract_file = File::create(base.join(name))?;
        extract_file.write_all(data.as_ref())?;
    }
    Ok(())
}
