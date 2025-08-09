use std::alloc::{self, Layout, alloc, dealloc};
use std::fs::{self, File};
use std::io::{self, Read, Seek, Write};
use std::mem::{self, MaybeUninit, transmute};
use std::path::Path;
use std::ptr;
use std::slice;

use crate::pna;

#[derive(Debug, Clone)]
pub struct Info {
    length: u32,
    size: u32,
    point: *const u8,
}
#[derive(Debug, Clone)]
pub struct Entry {
    pub size: u32,
    pub address: u32,
    pub str_data: Box<str>,
}
impl Info {
    pub fn new(point: *const u8, length: u32, size: u32) -> Self {
        Self {
            point,
            length,
            size,
        }
    }
    pub fn parse(&self) -> (u32, Box<[Entry]>) {
        let mut pos = 0usize;
        let buf = unsafe { slice::from_raw_parts(self.point, self.size as usize) };

        let mut entries_uninit: Box<[MaybeUninit<Entry>]> = {
            let layout =
                alloc::Layout::array::<MaybeUninit<Entry>>(self.length as usize)
                    .unwrap();
            let ptr = unsafe { alloc::alloc(layout) as *mut MaybeUninit<Entry> };
            if ptr.is_null() {
                alloc::handle_alloc_error(layout);
            }
            unsafe {
                Box::from_raw(ptr::slice_from_raw_parts_mut(
                    ptr,
                    self.length as usize,
                ))
            }
        };
        let mut max: u32 = 0;

        let mut tmp = [0u8; 8];

        for i in 0..self.length as usize {
            tmp.copy_from_slice(&buf[pos..pos + 8]);
            let (size, address) = unsafe { transmute(tmp) };
            pos += 8;
            if max < size {
                max = size;
            }
            let start = pos;
            loop {
                #[cfg(debug_assertions)]
                debug_assert!(pos <= self.size as usize);
                if buf[pos..pos + 2] == [0x0, 0x0] {
                    pos += 2;
                    break;
                }
                pos += 2;
            }
            let str_bytes = &buf[start..pos - 2];

            let u16_slice = unsafe {
                let ptr = str_bytes.as_ptr().cast::<u16>();
                slice::from_raw_parts(ptr, str_bytes.len() / 2)
            };
            let string = char::decode_utf16(u16_slice.iter().copied())
                .collect::<Result<String, _>>()
                .map_err(|e| e.to_string())
                .unwrap();

            entries_uninit[i] = MaybeUninit::new(Entry {
                str_data: string.into_boxed_str(),
                address,
                size,
            });
        }
        let entries = unsafe {
            mem::transmute::<Box<[MaybeUninit<Entry>]>, Box<[Entry]>>(entries_uninit)
        };

        (max, entries)
    }

    pub fn free(&self) {
        let layout = Layout::array::<u8>(self.size as usize).unwrap();
        unsafe {
            dealloc(self.point as *mut u8, layout);
        }
    }
}

pub fn extract(
    file: &Path,
    base: &Path,
    enable_sub_extract: bool,
) -> io::Result<()> {
    let mut file = File::open(file)?;
    let mut buffer = [0u8; 8];
    file.read_exact(&mut buffer)?;
    let (length, size): (u32, u32) = unsafe { mem::transmute(buffer) };

    let layout = Layout::array::<u8>(size as usize).unwrap();
    let ptr = unsafe { alloc(layout) };
    let buf = unsafe { slice::from_raw_parts_mut(ptr, size as usize) };
    file.read_exact(buf)?;
    let info = Info::new(ptr, length, size);

    let (max, entry) = info.parse();
    let mut buffer: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(max as usize);
    let mut file_path;
    #[cfg(debug_assertions)]
    let mut pos = 8 + size as u64;

    for i in 0..length as usize {
        #[cfg(debug_assertions)]
        debug_assert_eq!(pos, file.stream_position()?);

        let size = entry[i].size;
        #[cfg(debug_assertions)]
        {
            pos += size as u64;
        }
        let name = &entry[i].str_data;
        let pointer = unsafe {
            slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, size as usize)
        };
        file.read_exact(pointer)?;
        if enable_sub_extract && pointer[..4] == *b"PNAP" {
            let new_base = base.join(name.as_ref());
            fs::create_dir_all(&new_base)?;
            let sub_length =
                u32::from_le_bytes(pointer[16..20].try_into().unwrap()) as usize;
            let len = 20 + sub_length * 40;
            let struct_bytes = &pointer[20..len];
            let pna_info = unsafe {
                slice::from_raw_parts(
                    struct_bytes.as_ptr() as *const pna::Info,
                    sub_length,
                )
            };
            let mut pos = len;
            for (index, info) in pna_info.iter().enumerate() {
                file_path = new_base.join(format!("{}.png", index));
                let mut output_file = File::create(file_path)?;

                output_file.write_all(&pointer[pos..pos + info.size as usize])?;

                pos += info.size as usize;
            }
            continue;
        }
        let initialized_slice: &mut [u8] = unsafe {
            slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, size as usize)
        };
        if name[name.len() - 4..] == *".ws2" {
            for byte in initialized_slice.iter_mut() {
                *byte = (*byte >> 2) | (*byte << 6);
            }
        }
        file_path = base.join(name.as_ref());
        let mut output_file = File::create(file_path)?;
        output_file.write_all(pointer)?;
    }
    info.free();
    Ok(())
}
