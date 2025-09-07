use std::ptr;

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
    pub fn get(&mut self, n: usize) -> &'a [u8] {
        let start = self.pos;
        self.pos += n;

        unsafe { &*(&self.buf[start..start + n] as *const [u8]) }
    }
    #[inline(always)]
    pub fn get_no_ahead(&mut self, n: usize) -> &'a [u8] {
        let start = self.pos;
        unsafe { &*(&self.buf[start..start + n] as *const [u8]) }
    }

    #[inline(always)]
    pub fn address(&self) -> *const u8 {
        ptr::addr_of!(self.buf[self.pos])
    }
}
