use std::io;
use std::io::Read;
use std::mem::MaybeUninit;
use std::path::Path;
use std::ptr;
use std::{fs::File, io::Write};

use encoding_rs::UTF_16LE;
use flate2::read::ZlibDecoder;
use memmap2::Mmap;

use crate::ptr::ReadNum;

pub struct XP3<'a> {
    name: &'a str,
    address: u64,
    size: u64,
    zsize: u64,
}

enum Data<'a> {
    Borrowed(&'a [u8]),
    Owned(Box<[u8]>),
}
impl<'a> Data<'a> {
    #[inline(always)]
    fn as_slice(&self) -> &[u8] {
        match self {
            | Data::Borrowed(s) => s,
            | Data::Owned(b) => b,
        }
    }
}
pub fn extract(mmap: Mmap, _base: &Path) -> io::Result<()> {
    let content = &mmap[..];
    let mut pos: u64 = content.read_unaligned(11);
    let offset = pos as usize;
    let temp: u32 = content.read_unaligned(offset);
    if temp == 0x80 {
        pos = content.read_unaligned(offset + 9);
    }
    let offset = pos as usize;
    let type_ = content[offset];
    let data;
    if type_ == 0x00 {
        let size: u64 = content.read_unaligned(offset + 1);
        data = Data::Borrowed(&content[offset + 9..offset + 9 + size as usize])
        //
    } else {
        let size: u64 = content.read_unaligned(offset + 1);
        let size = size as usize;
        let meta: u64 = content.read_unaligned(offset + 9);
        println!("{size}-{meta}");
        let mut _data: Box<[u8]> = z(
            &content[offset + 17..offset + 17 + size],
            meta as usize,
        );
        data = Data::Owned(_data);
    }

    let mut stream = ByteStream::new(data.as_slice());
    let len = stream.len();
    while stream.pos < len {
        let signature: u32 = stream.read();
        let size: u64 = stream.read();
        match signature {
            | 0x656C6946 => {
                //"File"
                let start = stream.pos;
                while stream.pos < start + size as usize {
                    let sec: u32 = stream.read();
                    let sec_size: u64 = stream.read();
                    match sec {
                        | 0x6f666e69 => {
                            // "info"
                            let encrypt = 0_u32 != stream.read();
                            let zsize: u64 = stream.read();
                            let size: u64 = stream.read();
                            let len: u16 = stream.read();
                            let str = stream.get(2 * len as usize);
                            let (name, ..) = UTF_16LE.decode(str);
                            //print!("{encrypt}");
                        },
                        | 0x6d676573 => {
                            // "segm"
                            let count = sec_size / 0x1C;
                            println!("{count}");
                            for i in 0..count {
                                let compressed = 0_u32 != stream.read();
                                let address: u64 = stream.read();
                                let size: u64 = stream.read();
                                let zsize: u64 = stream.read();
                            }
                        },
                        | 0x726c6461 => {
                            // "adlr"
                            let hash: u32 = stream.read();
                            //if sec_size == 4 {}
                        },
                        | _ => {},
                    }
                }
            },
            | _ => {},
        }
    }
    Ok(())
}

pub fn z(buf: &[u8], size: usize) -> Box<[u8]> {
    let mut decoder = ZlibDecoder::new(buf);
    let mut buffer: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(size);
    let output = unsafe {
        &mut *ptr::slice_from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, size)
    };
    decoder.read_exact(output).unwrap_or_default();
    unsafe { buffer.assume_init() }
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
    fn skip(&mut self, n: usize) {
        self.pos += n;
    }

    #[inline(always)]
    fn seek(&mut self, n: usize) {
        self.pos = n;
    }

    #[inline(always)]
    const fn len(&self) -> usize {
        self.buf.len()
    }

    #[inline(always)]
    fn get(&mut self, n: usize) -> &'a [u8] {
        let start = self.pos;
        self.pos += n;

        unsafe { &*(&self.buf[start..start + n] as *const [u8]) }
    }
}
