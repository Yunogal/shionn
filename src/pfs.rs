use sha1::{Digest, Sha1};
use std::alloc::{self, Layout, alloc, dealloc};
use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::mem;
use std::path::Path;
use std::ptr;
use std::slice;
pub struct Info {
    length: u32,
    point: *const u8,
}
pub struct Entry<'a> {
    pub str_data: &'a str,
    pub address: u32,
    pub offset: u32,
}

impl Info {
    pub fn new(ptr: *const u8, length: u32) -> Self {
        Self { point: ptr, length }
    }

    fn read_u32_le_unchecked(&self, pos: usize) -> u32 {
        unsafe {
            let ptr = self.point.add(pos) as *const u32;
            u32::from_le(ptr.read_unaligned())
        }
    }
    pub fn parse<'a>(&'a self) -> (u32, u32, Box<[Entry<'a>]>) {
        let mut pos = 0usize;
        let buf = unsafe { slice::from_raw_parts(self.point, self.length as usize) };

        let size = u32::from_le_bytes(buf[pos..pos + 4].try_into().unwrap());
        pos += 4;

        let mut entries_uninit: Box<[mem::MaybeUninit<Entry>]> = {
            let layout =
                std::alloc::Layout::array::<mem::MaybeUninit<Entry>>(size as usize).unwrap();
            let ptr = unsafe { alloc::alloc(layout) as *mut mem::MaybeUninit<Entry> };
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            unsafe { Box::from_raw(ptr::slice_from_raw_parts_mut(ptr, size as usize)) }
        };
        let mut max: u32 = 0;
        for i in 0..size as usize {
            let str_length = u32::from_le_bytes(buf[pos..pos + 4].try_into().unwrap()) as usize;
            pos += 4;

            let str_bytes = &buf[pos..pos + str_length];
            pos += str_length;

            #[cfg(debug_assertions)]
            let str_data = std::str::from_utf8(str_bytes).unwrap();
            #[cfg(not(debug_assertions))]
            let str_data = unsafe { std::str::from_utf8_unchecked(str_bytes) };
            pos += 4;

            let address = u32::from_le_bytes(buf[pos..pos + 4].try_into().unwrap());
            pos += 4;

            let offset = u32::from_le_bytes(buf[pos..pos + 4].try_into().unwrap());
            pos += 4;
            if max < offset {
                max = offset
            }

            entries_uninit[i] = mem::MaybeUninit::new(Entry {
                str_data,
                address,
                offset,
            });
        }
        let entries = unsafe {
            mem::transmute::<Box<[mem::MaybeUninit<Entry>]>, Box<[Entry]>>(entries_uninit)
        };
        (max, size, entries)
    }

    pub fn sha1(&self) -> [u8; 20] {
        unsafe {
            let slice = slice::from_raw_parts(self.point, self.length as usize);
            let mut hasher = Sha1::new();
            hasher.update(slice);
            hasher.finalize().into()
        }
    }

    pub fn free(&self) {
        let layout = Layout::array::<u8>(self.length as usize).unwrap();
        unsafe {
            dealloc(self.point as *mut u8, layout);
        }
    }
}

pub fn extract(file: &Path, base: &Path) -> io::Result<()> {
    let mut file = File::open(file)?;
    let mut buffer = [0u8; 7];
    file.read_exact(&mut buffer)?;
    let (signature, size): ([u8; 3], [u8; 4]) = unsafe { mem::transmute(buffer) };

    if signature != *b"pf8" {
        return Ok(());
    }
    let length: u32 = unsafe { mem::transmute(size) };
    let layout = Layout::array::<u8>(length as usize).unwrap();

    let ptr = unsafe { alloc(layout) };
    let buf = unsafe { slice::from_raw_parts_mut(ptr, length as usize) };
    file.read_exact(buf)?;
    let info = Info::new(ptr, length as u32);
    let (max, length, entry) = info.parse();
    let mut current = entry[0].address;
    debug_assert_eq!(current as u64, file.stream_position()?);

    let mut file_buffer: Box<[mem::MaybeUninit<u8>]> = Box::new_uninit_slice(max as usize);
    let key = info.sha1();
    let mut file_path;
    for i in 0..length as usize {
        let address = entry[i].address;
        if address != current {
            file.seek(SeekFrom::Start(address as u64))?;
        }
        let size = entry[i].offset as usize;
        let raw_bytes =
            unsafe { slice::from_raw_parts_mut(file_buffer.as_mut_ptr() as *mut u8, size) };
        file.read_exact(raw_bytes)?;
        let initialized_slice: &mut [u8] =
            unsafe { slice::from_raw_parts_mut(file_buffer.as_mut_ptr() as *mut u8, size) };
        for (i, byte) in initialized_slice.iter_mut().enumerate() {
            *byte ^= key[i % 20];
        }
        file_path = base.join(entry[i].str_data);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut output_file = File::create(file_path)?;
        output_file.write_all(raw_bytes)?;
        current = size as u32 + address;
    }
    info.free();
    Ok(())
}
