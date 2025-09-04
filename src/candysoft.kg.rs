use std::ptr;

use crate::bmp;

#[repr(C)]
pub struct KG {
    pub signature: [u8; 4], // 'GCGK'
    pub width: u16,
    pub height: u16,
    pub size: u32,
}
#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<KG>(), 4);
    assert_eq!(size_of::<KG>(), 12);
}
pub fn extract(content: &mut [u8]) {
    let ptr: *const KG = content.as_ptr().cast();
    let &KG { width, height, .. } = unsafe { &*ptr };
    let height = height as usize;
    let width = width as usize;

    let ptr: *const u32 = unsafe { ptr.add(1).cast() };
    let table = unsafe { &*ptr::slice_from_raw_parts(ptr, height) };

    let size = height * width * 4;
    let mut buf = Box::<[u8]>::new_uninit_slice(size);
    let mut buf = unsafe {
        ptr::write_bytes(buf.as_mut_ptr(), 0, size);
        buf.assume_init()
    };
    let base = 12 + height * 4;
    let mut j = 0;
    let mut k = 0;
    for i in table {
        let mut pos = *i as usize + base;
        while k < width {
            let alpha = content[pos];
            let mut count = content[pos + 1] as u16;
            pos += 2;
            if count == 0 {
                count = 0x100;
            }
            if alpha == 0 {
                j += count as usize * 4;
            } else {
                for _ in 0..count {
                    buf[j + 3] = alpha;
                    buf[j + 2] = content[pos];
                    buf[j + 1] = content[pos + 1];
                    buf[j] = content[pos + 2];
                    pos += 3;
                    j += 4;
                }
            }
            k += count as usize;
        }
        k = 0;
    }
    bmp::save(
        width as u32,
        height as u32,
        "name.bmp",
        buf.as_ref(),
    )
    .unwrap();
}
