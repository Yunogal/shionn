use std::io::{self, BufWriter, Read};
use std::ptr;
use std::{fs::File, io::Write};

#[repr(C, packed)]
pub struct Bmp {
    pub signature: u16, //'BM'
    pub size: u32,
    pub zero: u32,
    pub address: u32,
    pub info_size: u32,
    pub width: u32,
    pub height: u32,
    pub planes: u16,
    pub bit_count: u16,
    pub type_: u32,
    pub data_size: u32,
    pub xpixels_per_meter: u32,
    pub ypixels_per_meter: u32,
    pub color_used: u32,
    pub color_important: u32,
}
impl Bmp {
    #[inline(always)]
    pub fn new(width: u32, height: u32, len: u32) -> Self {
        let bit_count = 8 * (len / height / width) as u16;
        Bmp {
            signature: 0x4D42,
            size: len + 0x36,
            zero: 0,
            address: 0x36,
            info_size: 0x28,
            width,
            height,
            planes: 1,
            bit_count,
            type_: 0,
            data_size: len,
            xpixels_per_meter: 0,
            ypixels_per_meter: 0,
            color_used: 0,
            color_important: 0,
        }
    }
}
#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Bmp>(), 1);
    assert_eq!(size_of::<Bmp>(), 0x36);
}
pub fn save(width: u32, height: u32, name: &str, data: &[u8]) -> io::Result<()> {
    let bmp = Bmp::new(width, height, data.len() as u32);
    let ptr = ptr::addr_of!(bmp);
    let file = File::create(name)?;
    let mut writer = BufWriter::new(file);
    let header_bytes: &[u8] =
        unsafe { &*ptr::slice_from_raw_parts(ptr as *const u8, 0x36) };
    writer.write_all(header_bytes)?;
    let row_size = width * 4;
    for y in (0..height).rev() {
        let start = (y * row_size) as usize;
        let end = start + row_size as usize;
        writer.write_all(&data[start..end])?;
    }
    Ok(())
}
