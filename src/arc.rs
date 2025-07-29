use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::mem;
use std::path::Path;
use std::ptr;
use std::slice;

#[derive(Debug, Clone)]
pub struct Info {
    pub size: u32,
    pub address: u32,
    pub name: Box<str>,
}

pub fn extract(file: &Path, base: &Path) -> io::Result<()> {
    let mut file = File::open(file)?;
    let mut buffer = [0u8; 8];
    file.read_exact(&mut buffer)?;
    let (length, size): (u32, u32) = unsafe { mem::transmute(buffer) };
    let mut buffers: Box<[mem::MaybeUninit<u8>]> = Box::new_uninit_slice(size as usize);
    let raw_bytes =
        unsafe { slice::from_raw_parts_mut(buffers.as_mut_ptr() as *mut u8, size as usize) };
    file.read_exact(raw_bytes)?;
    let initialized_buffers: Box<[u8]> = unsafe { buffers.assume_init() };

    let mut str = [0u8; 50];
    fs::create_dir_all(base)?;
    let mut ptr = initialized_buffers.as_ptr() as *mut u8;
    for _i in 0..length {
        let bytes: [u8; 8] = unsafe { ptr::read_unaligned(ptr as *const [u8; 8]) };
        let (size, _address): (u32, u32) = unsafe { mem::transmute(bytes) };
        ptr = unsafe { ptr.add(8) };
        for i in 0..50 {
            let two_bytes: [u8; 2] = unsafe { ptr::read_unaligned(ptr as *const [u8; 2]) };
            ptr = unsafe { ptr.add(2) };

            if two_bytes == [0, 0] {
                str[0] = i as u8;
                break;
            }
            str[i + 1] = two_bytes[0];
        }
        let end = str[0] as usize;
        let slice = &str[1..=end];
        #[cfg(debug_assertions)]
        let str = std::str::from_utf8(slice).unwrap();
        #[cfg(not(debug_assertions))]
        let str = unsafe { std::str::from_utf8_unchecked(slice) };

        let mut buffers: Box<[mem::MaybeUninit<u8>]> = Box::new_uninit_slice(size as usize);
        let raw_bytes =
            unsafe { slice::from_raw_parts_mut(buffers.as_mut_ptr() as *mut u8, size as usize) };
        file.read_exact(raw_bytes)?;
        let mut data: Box<[u8]> = unsafe { buffers.assume_init() };

        //decode func
        if &str[str.len() - 4..] == ".ws2" {
            for byte in data.iter_mut() {
                *byte = (*byte >> 2) | (*byte << 6);
            }
        }

        let mut output_file = File::create(base.join(str))?;
        output_file.write_all(&data)?;
    }

    Ok(())
}
