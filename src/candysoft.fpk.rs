use std::mem::transmute;

#[repr(transparent)]
pub struct FPK {
    pub count: u32,
}
//30 26 B2 75 8E 66 CF 11

#[repr(C)]
pub struct Entry {
    pub address: u32,
    pub size: u32,
    pub name: [u8; 0x18],
    pub serial: u32,
}
impl Entry {
    pub fn name(&self) -> &str {
        let mut len: usize = 0;
        for i in self.name {
            if i != 0x00 {
                len += 1;
            }
        }
        unsafe { transmute(&self.name[..len]) }
    }
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<FPK>(), 4);
    assert_eq!(size_of::<FPK>(), 4);
    assert_eq!(align_of::<Entry>(), 4);
    assert_eq!(size_of::<Entry>(), 36);
}

pub fn extract(content: &mut [u8]) -> &[u8] {
    if content[..8] == *b"\x30\x26\xb2\x75\x8e\x66\xcf\x11" {
        // Advanced Systems Format
        // WMV
        return content;
    }
    let ptr = content.as_ptr();
    let mut count = unsafe { *ptr.cast::<u32>() };
    count &= 0x7fffffff;

    let len = content.len();
    let key = unsafe {
        ptr.add(len - 8)
            .cast::<[u8; 4]>()
            .read_unaligned()
    };
    let address =
        unsafe { ptr.add(len - 4).cast::<u32>().read_unaligned() } as usize;
    let size = count as usize * size_of::<Entry>();
    let meta = &mut content[address..address + size];
    for (index, i) in meta.iter_mut().enumerate() {
        *i ^= key[index & 3];
    }
    meta
}
