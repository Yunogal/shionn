#[inline]
pub const fn as_u32_unaligned(slice: &[u8]) -> u32 {
    let ptr = slice.as_ptr() as *const u32;
    unsafe { ptr.read_unaligned() }
}

#[inline]
pub const fn as_u32(slice: &[u8]) -> u32 {
    unsafe { *(slice.as_ptr() as *const u32) }
}

#[inline]
pub const fn as_u16_unaligned(slice: &[u8]) -> u16 {
    let ptr = slice.as_ptr() as *const u16;
    unsafe { ptr.read_unaligned() }
}

#[inline]
pub const fn as_u16(slice: &[u8]) -> u16 {
    unsafe { *(slice.as_ptr() as *const u16) }
}

#[test]
fn ptr() {
    let slice = [
        0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7,
        0x8, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6,
        0x7, 0x8, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8,
    ];
    let a = as_u32(&slice[0..]);
    assert_eq!(a, 0x04030201);
    // let b = as_u32(&slice[1..]); // ub

    let a = as_u16(&slice[0..]);
    assert_eq!(a, 0x0201);
    let b = as_u16_unaligned(&slice[1..]);
    assert_eq!(b, 0x0302);
    // let b = as_u16(&slice[1..]); //ub
}

pub trait ReadNum: AsRef<[u8]> {
    #[inline(always)]
    fn read<T: Copy>(&self, pos: usize) -> T {
        unsafe { *(self.as_ref().as_ptr().add(pos) as *const T) }
    }

    #[inline(always)]
    fn read_unaligned<T: Copy>(&self, pos: usize) -> T {
        unsafe {
            let ptr = self.as_ref().as_ptr().add(pos) as *const T;
            ptr.read_unaligned()
        }
    }
}
impl ReadNum for [u8] {}
