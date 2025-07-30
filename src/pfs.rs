use sha1::{Digest, Sha1};
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
pub fn extract(file: &Path, base: &Path) -> io::Result<()> {
    let mut file = File::open(file)?;
    let mut buffer = [0u8; 3];
    file.read_exact(&mut buffer)?;
    if buffer != *b"pf8" {
        return Ok(());
    }
    let mut size_ = [0u8; 8];
    file.read_exact(&mut size_)?;
    let (size, length): (u32, u32) = unsafe { mem::transmute(size_) };

    file.seek(SeekFrom::Start(7))?;
    let mut buffer: Box<[mem::MaybeUninit<u8>]> = Box::new_uninit_slice(size as usize);
    let raw_bytes =
        unsafe { slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, size as usize) };
    file.read_exact(raw_bytes)?;
    let buffer_: Box<[u8]> = unsafe { buffer.assume_init() };
    let mut hasher = Sha1::new();
    hasher.update(&buffer_);
    let key: [u8; 20] = hasher.finalize().into();

    let mut ptr = buffer_.as_ptr() as *mut u8;
    let mut file_path;
    file.seek(SeekFrom::Start((size + 7) as u64))?;
    ptr = unsafe { ptr.add(4) };
    for i in 0..length {
        let bytes: [u8; 4] = unsafe { ptr::read_unaligned(ptr as *const [u8; 4]) };
        let str_length: u32 = unsafe { mem::transmute(bytes) };

        ptr = unsafe { ptr.add(4) };
        let slice = unsafe { slice::from_raw_parts(ptr, str_length as usize) };

        #[cfg(debug_assertions)]
        let str = std::str::from_utf8(slice).unwrap();
        #[cfg(not(debug_assertions))]
        let str = unsafe { std::str::from_utf8_unchecked(slice) };
        ptr = unsafe { ptr.add((str_length + 4) as usize) };
        let bytes: [u8; 8] = unsafe { ptr::read_unaligned(ptr as *const [u8; 8]) };
        let (address, offset): (u32, u32) = unsafe { mem::transmute(bytes) };
        if address as u64 != file.stream_position()? {
            file.seek(SeekFrom::Start(address as u64))?;
        };
        ptr = unsafe { ptr.add(8) };

        let mut file_buffer: Box<[mem::MaybeUninit<u8>]> = Box::new_uninit_slice(offset as usize);
        let raw_bytes = unsafe {
            slice::from_raw_parts_mut(file_buffer.as_mut_ptr() as *mut u8, offset as usize)
        };
        file.read_exact(raw_bytes)?;

        let mut file_buffer_: Box<[u8]> = unsafe { file_buffer.assume_init() };

        for (i, byte) in file_buffer_.iter_mut().enumerate() {
            *byte ^= key[i % key.len()];
        }

        file_path = base.join(&str);

        if let Some(parent) = base.join(&str).parent() {
            fs::create_dir_all(parent)?;
        }
        let mut output_file = File::create(file_path)?;
        output_file.write_all(&file_buffer_)?;
    }
    Ok(())
}
