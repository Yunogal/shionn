use std::borrow::Cow;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::ptr;

use encoding_rs::UTF_16LE;
use flate2::read::ZlibDecoder;

use crate::kirikiri::*;

#[repr(C, packed)]
pub struct Xp3 {
    signature: [u8; 11], //'XP3\x0D\x0A\x20\x0A\x1A\x8B\x67\x01'
    address: u64,
}

#[repr(C, packed)]
pub struct Xp3Tail {
    pub type_: u8,
    pub size: u64,
    pub zsize: u64,
}

//OR

#[repr(C, packed)]
pub struct Xp3Tail_ {
    pub type_: u8,
    pub size: u64,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Xp3>(), 1);
    assert_eq!(size_of::<Xp3>(), 19);
    assert_eq!(align_of::<Xp3Tail>(), 1);
    assert_eq!(size_of::<Xp3Tail>(), 17);
    assert_eq!(align_of::<Xp3Tail_>(), 1);
    assert_eq!(size_of::<Xp3Tail_>(), 9);
}

pub fn general(
    content: &mut [u8],
    base: &Path,
    game: impl Filter,
) -> io::Result<()> {
    let mut pos = unsafe {
        let ptr: *const u64 = content.as_ptr().add(11).cast();
        ptr.read_unaligned()
    };
    let addr = ptr::addr_of!(content[pos as usize]);
    if unsafe { *addr } == 0x80 {
        pos = unsafe {
            let ptr: *const u64 = addr.add(9).cast();
            ptr.read_unaligned()
        };
    }
    let ptr = ptr::addr_of!(content[pos as usize]);

    let type_ = unsafe { ptr.read_unaligned() };
    let size = unsafe { ptr.add(1).cast::<u64>().read_unaligned() } as usize;

    let offset = pos as usize;
    let data;
    if type_ == 0x00 {
        let slice = unsafe {
            let ptr = ptr.add(9);
            &*ptr::slice_from_raw_parts(ptr, size)
        };
        data = Cow::Borrowed(slice);
    } else {
        let zsize = unsafe { ptr.add(9).cast::<u64>().read_unaligned() } as usize;
        let mut buf: Vec<u8> = Vec::with_capacity(zsize);
        unsafe {
            buf.set_len(zsize);
        }

        let mut decoder =
            ZlibDecoder::new(&content[offset + 17..offset + 17 + size]);
        decoder.read_exact(&mut buf)?;
        data = Cow::Owned(buf);
    }
    let mut stream = ByteStream::new(data.as_ref());
    let len = stream.len();
    let mut buf: Vec<u8> = Vec::with_capacity(2560 * 1440 * 4); //14.0625MB
    while stream.pos < len {
        let signature: u32 = stream.read();
        let _size: u64 = stream.read();
        match signature {
            | 0x656C6946 => {
                //"File"
                let info: u32 = stream.read();
                debug_assert_eq!(info, 0x6f666e69);
                let _info_size: u64 = stream.read();
                let _encrypt = 0_u32 != stream.read();
                let zsize: u64 = stream.read();
                let size: u64 = stream.read();
                let len: u16 = stream.read();
                let str = stream.get(2 * len as usize);
                let (name, ..) = UTF_16LE.decode(str);
                let segm: u32 = stream.read();
                debug_assert_eq!(segm, 0x6d676573);
                let _segm_size: u64 = stream.read();
                //let count = segm_size / 0x1C;
                //for i in 0..count {}
                let compressed = 0_u32 != stream.read();
                let address: u64 = stream.read();
                let _zsize: u64 = stream.read();
                let _size: u64 = stream.read();
                let adlr: u32 = stream.read();
                debug_assert_eq!(adlr, 0x726c6461);
                let adlr_size: u64 = stream.read();
                debug_assert_eq!(adlr_size, 4);
                let hash: u32 = stream.read();
                if name.len() > 0x100 {
                    continue;
                }
                let start = address as usize;
                let end = start + size as usize;
                let path = base.join(name.as_ref());
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut file = File::create(path)?;
                if compressed {
                    let mut decoder = ZlibDecoder::new(&content[start..end]);
                    unsafe {
                        buf.set_len(zsize as usize);
                    }
                    decoder.read_exact(&mut buf)?;
                    game.filter(&mut buf, hash);
                    file.write_all(&buf)?;
                } else {
                    game.filter(&mut content[start..end], hash);
                    file.write_all(&content[start..end])?;
                }
            },
            | _ => {},
        }
    }
    Ok(())
}

pub struct ByteStream<'a> {
    pub buf: &'a [u8],
    pub pos: usize,
}

impl<'a> ByteStream<'a> {
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
    pub fn skip(&mut self, n: usize) {
        self.pos += n;
    }

    #[inline(always)]
    pub fn seek(&mut self, n: usize) {
        self.pos = n;
    }

    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.buf.len()
    }

    #[inline(always)]
    fn get(&mut self, n: usize) -> &'a [u8] {
        let start = self.pos;
        self.pos += n;

        unsafe { &*(&self.buf[start..start + n] as *const [u8]) }
    }

    #[inline(always)]
    pub fn address(&self) -> *const u8 {
        ptr::addr_of!(self.buf[self.pos])
    }
}
