use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::mem::{self, MaybeUninit};
use std::path::Path;
use std::slice;

#[derive(Debug, Clone, Copy)]
pub struct Info {
    pub unknown0: u32,
    pub unknown1: u32,
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: u32,
    pub height: u32,
    pub unknown2: u32,
    pub unknown3: u32,
    pub unknown4: u32,
    pub size: u32,
}

pub fn extract(file: &Path, base: &Path) -> io::Result<()> {
    let new_base = base.join(file);
    let file_name = file.file_stem().unwrap().to_str().unwrap();
    fs::create_dir_all(&new_base)?;
    let mut file = File::open(file)?;
    let mut buffer = [0u8; 20];
    file.read_exact(&mut buffer)?;
    let (signature, _, _, _, length): (u32, u32, u32, u32, u32) =
        unsafe { mem::transmute(buffer) };
    if signature != 0x50414e50 {
        return Ok(());
    }
    let mut buffer: Box<[MaybeUninit<u8>]> =
        Box::new_uninit_slice(40 * length as usize);
    let raw_bytes = unsafe {
        slice::from_raw_parts_mut(
            buffer.as_mut_ptr() as *mut u8,
            40 * length as usize,
        )
    };
    file.read_exact(raw_bytes)?;
    let info_ptr = Box::into_raw(buffer) as *mut Info;
    let info: Box<[Info]> = unsafe {
        Box::from_raw(slice::from_raw_parts_mut(
            info_ptr,
            length as usize,
        ))
    };
    let max = info
        .iter()
        .map(|info| info.size)
        .max()
        .unwrap_or(4 * 1024 * 1024);
    let mut data: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(max as usize);
    for (index, info) in info.iter().enumerate() {
        let data = unsafe {
            slice::from_raw_parts_mut(
                data.as_mut_ptr() as *mut u8,
                info.size as usize,
            )
        };
        file.read_exact(data)?;
        let name = format!("{file_name}{index:03}.png");
        let mut output_file = File::create(new_base.join(name))?;
        output_file.write_all(data)?;
    }
    Ok(())
}
