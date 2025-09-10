use std::ptr;

pub struct WriteStream<'a> {
    pub buf: &'a mut [u8],
    pub pos: usize,
}
impl<'a> WriteStream<'a> {
    #[inline(always)]
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }
    pub fn copy_from(&mut self, count: usize, src: &mut ReadStream) {
        let ptr = unsafe { self.buf.as_mut_ptr().add(self.pos) };
        self.pos += count;
        let srcs = src.address();
        src.pos += count;
        unsafe { ptr::copy_nonoverlapping(srcs, ptr, count) };
    }
    pub fn copy_from_self(&mut self, offset: usize, mut remaining: usize) {
        let start = self.pos - offset;
        let mut src = ptr::addr_of!(self.buf[start]);
        let mut dst = ptr::addr_of_mut!(self.buf[self.pos]);
        loop {
            let chunk = offset.min(remaining);
            unsafe {
                ptr::copy_nonoverlapping(src, dst, chunk);
                src = src.add(chunk);
                dst = dst.add(chunk);
            }
            self.pos += chunk;
            remaining -= chunk;
            if remaining == 0 {
                break;
            }
        }
        // while remaining > 0 {
        //     let chunk = offset.min(remaining);
        //     self.buf
        //         .copy_within(start..start + chunk, self.pos);
        //     self.pos += chunk;
        //     remaining -= chunk
        // }
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
