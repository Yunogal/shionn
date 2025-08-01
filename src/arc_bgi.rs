use std::fs::File;
use std::io::{self, Read, Write};
use std::mem::{MaybeUninit, transmute};
use std::path::Path;
use std::slice;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(4))]
pub struct Info {
    pub name: [u8; 0x60],
    pub address: u32,
    pub size: u32,
    pub useless: [u8; 24],
}

impl Info {
    pub fn name(&self) -> &str {
        let len = self.name.iter().position(|&c| c == 0).unwrap_or(96);
        str::from_utf8(&self.name[..len]).unwrap_or(" ")
    }
}

pub fn extract(file: &Path, base: &Path) -> io::Result<()> {
    let mut file = File::open(file)?;
    let mut buffer = [0u8; 16];
    file.read_exact(&mut buffer)?;
    let (signature, length): ([u8; 12], u32) = unsafe { transmute(buffer) };
    if signature != *b"BURIKO ARC20" {
        return Ok(());
    }

    let mut buffer: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(0x80 * length as usize);
    let pointer = unsafe {
        slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, 0x80 * length as usize)
    };

    file.read_exact(pointer)?;
    let info_ptr = Box::into_raw(buffer) as *mut Info;
    let info: Box<[Info]> =
        unsafe { Box::from_raw(slice::from_raw_parts_mut(info_ptr, length as usize)) };
    let max = info
        .iter()
        .map(|info| info.size)
        .max()
        .unwrap_or(4 * 1024 * 1024);

    let mut data: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(max as usize);
    for info in info.iter() {
        let content =
            unsafe { slice::from_raw_parts_mut(data.as_mut_ptr() as *mut u8, info.size as usize) };
        file.read_exact(content)?;
        let mut output_file = File::create(base.join(info.name()))?;
        output_file.write_all(content)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;
    #[test]
    fn size() {
        assert_eq!(size_of::<Info>(), 0x80);
    }
}
