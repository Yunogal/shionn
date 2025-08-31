use std::ptr;

#[repr(C, packed)]
pub struct G00_0 {
    pub type_: u8,
    pub width: u16,
    pub height: u16,
    pub size: u32,
    pub zsize: u32,
}

#[repr(C, packed)]
pub struct G00_2 {
    pub type_: u8,
    pub width: u16,
    pub height: u16,
    pub count: u32, //'1'
    pub x: u32,
    pub y: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub size: u32,
    pub zsize: u32,
}

#[repr(C, packed)]
pub struct G00_3 {
    pub type_: u8,
    pub width: u16,
    pub height: u16,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<G00_0>(), 1);
    assert_eq!(size_of::<G00_0>(), 0x0D);
    assert_eq!(align_of::<G00_2>(), 1);
    assert_eq!(size_of::<G00_2>(), 0x29);
    assert_eq!(align_of::<G00_3>(), 1);
    assert_eq!(size_of::<G00_3>(), 0x05);
}

static KEY: [u8; 256] = [
    0x45, 0x0C, 0x85, 0xC0, 0x75, 0x14, 0xE5, 0x5D, 0x8B, 0x55, 0xEC, 0xC0, 0x5B,
    0x8B, 0xC3, 0x8B, 0x81, 0xFF, 0x00, 0x00, 0x04, 0x00, 0x85, 0xFF, 0x6A, 0x00,
    0x76, 0xB0, 0x43, 0x00, 0x76, 0x49, 0x00, 0x8B, 0x7D, 0xE8, 0x8B, 0x75, 0xA1,
    0xE0, 0x0C, 0x85, 0xC0, 0xC0, 0x75, 0x78, 0x30, 0x44, 0x00, 0x85, 0xFF, 0x76,
    0x37, 0x81, 0x1D, 0xD0, 0xFF, 0x00, 0x00, 0x75, 0x44, 0x8B, 0xB0, 0x43, 0x45,
    0xF8, 0x8D, 0x55, 0xFC, 0x52, 0x00, 0x76, 0x68, 0x00, 0x00, 0x04, 0x00, 0x6A,
    0x43, 0x8B, 0xB1, 0x43, 0x00, 0x6A, 0x05, 0xFF, 0x50, 0xFF, 0xD3, 0xA1, 0xE0,
    0x04, 0x00, 0x56, 0x15, 0x2C, 0x44, 0x00, 0x85, 0xC0, 0x74, 0x09, 0xC3, 0xA1,
    0x5F, 0x5E, 0x33, 0x8B, 0xE5, 0x5D, 0xE0, 0x30, 0x04, 0x00, 0x81, 0xC6, 0x00,
    0x00, 0x81, 0xEF, 0x04, 0x00, 0x85, 0x30, 0x44, 0x00, 0x00, 0x00, 0x5D, 0xC3,
    0x8B, 0x55, 0xF8, 0x8D, 0x5E, 0x5B, 0x4D, 0xFC, 0x51, 0xC4, 0x04, 0x5F, 0x8B,
    0xE5, 0x43, 0x00, 0xEB, 0xD8, 0x8B, 0x45, 0xFF, 0x15, 0xE8, 0x83, 0xC0, 0x57,
    0x56, 0x52, 0x2C, 0xB1, 0x01, 0x00, 0x8B, 0x7D, 0xE8, 0x89, 0x00, 0xE8, 0x45,
    0xF4, 0x8B, 0x20, 0x50, 0x6A, 0x47, 0x28, 0x00, 0x50, 0x53, 0xFF, 0x15, 0x34,
    0xE4, 0x6A, 0xB1, 0x43, 0x00, 0x0C, 0x8B, 0x45, 0x00, 0x6A, 0x8B, 0x4D, 0xEC,
    0x89, 0x08, 0x8A, 0x85, 0xC0, 0x45, 0xF0, 0x84, 0x8B, 0x45, 0x10, 0x74, 0x05,
    0xF5, 0x28, 0x01, 0x00, 0x83, 0xC4, 0x52, 0x6A, 0x08, 0x89, 0x45, 0x83, 0xC2,
    0x20, 0x00, 0xE8, 0xE8, 0xF4, 0xFB, 0xFF, 0xFF, 0x8B, 0x8B, 0x5D, 0x45, 0x0C,
    0x83, 0xC0, 0x74, 0xC5, 0xF8, 0x53, 0xC4, 0x08, 0x85, 0xC0, 0x75, 0x56, 0x30,
    0x44, 0x8B, 0x1D, 0xD0, 0xF0, 0xA1, 0xE0, 0x00, 0x83,
];

pub fn parse(content: &mut [u8], output: &mut [u8]) {
    match content[0] {
        | 0 => {
            let g00: *const G00_0 = content.as_ptr().cast();
            let _g00 = unsafe { &*g00 };
            lz(&content[0x0D..], output, 1, 3);
        },
        | 2 => {
            let g00: *const G00_2 = content.as_ptr().cast();
            let g00 = unsafe { &*g00 };
            if g00.count != 1 {
                return;
            }
            lz(&content[0x29..], output, 2, 1);
        },
        | 3 => {
            let g00: *const G00_2 = content.as_ptr().cast();
            let _g00 = unsafe { &*g00 };
            for (index, u) in &mut content[5..].iter_mut().enumerate() {
                *u ^= KEY[index & 255];
            }
        },
        | 1 => {},
        | _ => {},
    }
}

fn lz(read: &[u8], write: &mut [u8], num: u32, bpp: u32) {
    let len = read.len();
    let mut read = ReadStream::new(read);
    let mut write = WriteStream::new(write);
    let mut token: u16;
    let mut offset;
    let mut count;
    let mut control: u16 = 2;
    while read.pos < len {
        control >>= 1;
        if control == 1 {
            let bit: u8 = read.read_aligned();
            control = bit as u16 | 0x100;
        }
        if control & 1 == 1 {
            write.copy_from(&mut read, bpp as usize);
        } else {
            token = read.read();

            offset = (token >> 4) as usize * bpp as usize;
            count = ((token & 0xF) + num as u16) as usize * bpp as usize;
            write.copy_from_self(offset, count);
        }
    }
}
pub struct ReadStream<'a> {
    pub buf: &'a [u8],
    pub pos: usize,
}

impl<'a> ReadStream<'a> {
    #[inline(always)]
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }
    #[inline(always)]
    pub const fn read_aligned<T: Copy>(&mut self) -> T {
        let temp = unsafe { *(self.buf.as_ptr().add(self.pos) as *const T) };
        self.pos += size_of::<T>();
        temp
    }

    #[inline(always)]
    pub const fn read<T: Copy>(&mut self) -> T {
        let ptr = unsafe { self.buf.as_ptr().add(self.pos) as *const T };
        self.pos += size_of::<T>();
        unsafe { ptr.read_unaligned() }
    }
    #[inline(always)]
    fn address(&self) -> *const u8 {
        ptr::addr_of!(self.buf[self.pos])
    }
}
pub struct WriteStream<'a> {
    pub buf: &'a mut [u8],
    pub pos: usize,
}

impl<'a> WriteStream<'a> {
    #[inline(always)]
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn copy_from(&mut self, src: &mut ReadStream, bpp: usize) {
        let ptr = unsafe { self.buf.as_ptr().add(self.pos) as *mut u8 };
        self.pos += bpp;
        let srcs = src.address();
        src.pos += bpp;
        unsafe { ptr::copy_nonoverlapping(srcs, ptr, bpp) };
    }
    pub fn copy_from_self(&mut self, offset: usize, mut remaining: usize) {
        let src_base = self.pos - offset;
        let mut dst = self.pos;

        while remaining > 0 {
            let avail = dst - src_base;
            let chunk = avail.min(remaining);
            self.buf
                .copy_within(src_base..src_base + chunk, dst);
            dst += chunk;
            remaining -= chunk;
        }
        self.pos = dst;
    }
}
