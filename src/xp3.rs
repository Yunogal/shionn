use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::mem::MaybeUninit;
use std::path::Path;
use std::ptr;

use encoding_rs::UTF_16LE;
use flate2::read::ZlibDecoder;

use crate::ptr::ReadNum;
use memmap2::Mmap;

pub enum Data<'a> {
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

    fn as_mut(&mut self) -> &mut [u8] {
        if let Data::Borrowed(s) = self {
            *self = Data::Owned(s.to_owned().into_boxed_slice());
        }
        match self {
            | Data::Owned(b) => b.as_mut(),
            | Data::Borrowed(_) => unreachable!(),
        }
    }
}

pub fn extract(mmap: Mmap, base: &Path) -> io::Result<()> {
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
    } else {
        let size: u64 = content.read_unaligned(offset + 1);
        let size = size as usize;
        let meta: u64 = content.read_unaligned(offset + 9);
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
                let info: u32 = stream.read();
                debug_assert_eq!(info, 0x6f666e69);
                let _info_size: u64 = stream.read();
                let encrypt = 0_u32 != stream.read();
                let size: u64 = stream.read();
                let zsize: u64 = stream.read();
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
                let _size: u64 = stream.read();
                let _zsize: u64 = stream.read();

                let adlr: u32 = stream.read();
                debug_assert_eq!(adlr, 0x726c6461);
                let adlr_size: u64 = stream.read();
                debug_assert_eq!(adlr_size, 4);
                let hash: u32 = stream.read();
                if name.len() > 0x100 {
                    continue;
                }
                //
                let start = address as usize;
                let end = start + zsize as usize;
                let mut data;
                if compressed {
                    data = Data::Owned(z(&content[start..end], size as usize))
                } else {
                    data = Data::Borrowed(&content[start..end])
                }
                let content = if encrypt {
                    let key = generate_key(hash);
                    let data = data.as_mut();
                    xor_bytes_in_place(data, &key);
                    Data::Borrowed(data)
                } else {
                    data
                };
                let path = base.join(name.as_ref());
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?; // 递归创建目录 a/b/c
                }
                let mut file = File::create(path)?;
                file.write_all(content.as_slice())?;
            },
            | _ => {},
        }
    }
    Ok(())
}
pub fn xor_bytes_in_place(data: &mut [u8], key: &[u8]) {
    for (i, byte) in data.iter_mut().enumerate() {
        *byte ^= key[i % 31];
    }
}
fn generate_key(mut hash: u32) -> [u8; 32] {
    hash &= 0x7fffffff;
    hash = (hash << 31) | hash;

    let mut key: [MaybeUninit<u8>; 32] =
        unsafe { MaybeUninit::uninit().assume_init() };

    for i in 0..31 {
        key[i] = MaybeUninit::new(hash as u8);
        hash = (hash & 0xfffffffe) << 23 | hash >> 8;
    }
    key[31] = MaybeUninit::new(0);
    unsafe { std::mem::transmute::<[MaybeUninit<u8>; 32], [u8; 32]>(key) }
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

    #[inline(always)]
    fn address(&self) -> *const u8 {
        ptr::addr_of!(self.buf[self.pos])
    }
}
