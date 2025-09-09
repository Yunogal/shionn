use std::fs::File;
use std::io::{Result, Write};
use std::mem::{MaybeUninit, transmute};
use std::path::Path;
use std::ptr;

#[repr(C)]
pub struct Pac {
    pub signature: [u8; 3], // 'PAC'
    pub ubk: u8,
    pub count: u32,
    pub type_: u32,
}

#[repr(C)]
struct Entry {
    pub name: [u8; 0x40],
    pub address: u32,
    pub zsize: u32,
    pub size: u32,
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
    assert_eq!(align_of::<Pac>(), 4);
    assert_eq!(size_of::<Pac>(), 12);
    assert_eq!(align_of::<Entry>(), 4);
    assert_eq!(size_of::<Entry>(), 0x4C);
}

pub fn extract(content: &mut [u8], base: &Path) -> Result<()> {
    let ptr = content.as_ptr();
    let pac: *const Pac = ptr.cast();
    let &Pac { count, type_, .. } = unsafe { &*pac };
    let end = content.len() - 4;
    let length = unsafe { ptr.add(end).cast::<u32>().read_unaligned() } as usize;
    let header = &mut content[end - length..end];
    for i in header.iter_mut() {
        *i = !*i;
    }
    let mut reader = BitReader::new(header);
    let tree = parse_tree(&mut reader).expect("Failed to parse tree");
    let decoded = decode_data(&tree, &mut reader, 0x4C * count as usize);

    let entry = unsafe {
        &*ptr::slice_from_raw_parts(decoded.as_ptr() as *const Entry, count as usize)
    };
    for i in entry {
        let mut file = File::create(base.join(i.name()))?;
        let start = i.address as usize;
        let end = start + i.size as usize;
        if i.size == i.zsize {
            file.write_all(&content[start..end])?;
        } else {
            //zstd or zlib
            file.write_all(&content[start..end])?;
        }
    }

    Ok(())
}

#[derive(Debug)]
enum HuffmanNode {
    Leaf(u8),
    Internal(Box<HuffmanNode>, Box<HuffmanNode>),
}

struct BitReader<'a> {
    data: &'a [u8],
    byte_pos: usize,
    bit_pos: u8,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            byte_pos: 0,
            bit_pos: 0,
        }
    }

    fn read_bit(&mut self) -> Option<u8> {
        if self.byte_pos >= self.data.len() {
            return None;
        }
        let byte = self.data[self.byte_pos];

        let bit = (byte >> (7 - self.bit_pos)) & 1;
        self.bit_pos += 1;
        if self.bit_pos == 8 {
            self.bit_pos = 0;
            self.byte_pos += 1;
        }
        Some(bit)
    }

    fn read_byte(&mut self) -> Option<u8> {
        let mut val = 0u8;
        for _ in 0..8 {
            val <<= 1;
            val |= self.read_bit()?;
        }
        Some(val)
    }
}

fn parse_tree(reader: &mut BitReader) -> Option<HuffmanNode> {
    let bit = reader.read_bit()?;
    if bit == 1 {
        let left = parse_tree(reader)?;
        let right = parse_tree(reader)?;
        Some(HuffmanNode::Internal(
            Box::new(left),
            Box::new(right),
        ))
    } else {
        let sym = reader.read_byte()?;
        Some(HuffmanNode::Leaf(sym))
    }
}

fn decode_data(
    root: &HuffmanNode,
    reader: &mut BitReader,
    output_len: usize,
) -> Box<[u8]> {
    let mut output: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(output_len);

    for i in 0..output_len {
        let mut node = root;
        loop {
            match node {
                | HuffmanNode::Leaf(sym) => {
                    output[i] = MaybeUninit::new(*sym);
                    break;
                },
                | HuffmanNode::Internal(left, right) => {
                    let bit = reader
                        .read_bit()
                        .expect("The bitstream ends before reaching a leaf (data/tree mismatch)");
                    node = if bit == 0 {
                        left.as_ref()
                    } else {
                        right.as_ref()
                    };
                },
            }
        }
    }

    unsafe { output.assume_init() }
}
