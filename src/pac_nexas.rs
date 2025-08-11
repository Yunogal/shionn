use std::fs::OpenOptions;
use std::io::{self, Write};
use std::mem;
use std::path::Path;
use std::ptr;

use memmap2::Mmap;

struct Info {
    pub name: [u8; 0x40],
    pub address: u32,
    pub size: u32,
    pub real_size: u32,
}

impl Info {
    pub fn name(&self) -> &str {
        let mut len: usize = 0;
        for i in self.name {
            if i != 0x00 {
                len += 1;
            }
        }
        unsafe { mem::transmute(&self.name[..len]) }
    }
}

#[test]
fn size() {
    use std::mem::size_of;

    assert_eq!(size_of::<Info>(), 0x4C);
}

pub fn extract(mmap: Mmap, base: &Path) -> io::Result<()> {
    let data = &mmap[4..12];
    let [count, _type] = unsafe {
        let ptr = data.as_ptr() as *const [u32; 2];
        *ptr
    };
    let len = mmap.len();
    let end = len - 4;
    let length = bytes_to_u32_le(&mmap[end..len]) as usize;
    let mut header: Box<[mem::MaybeUninit<u8>]> = Box::new_uninit_slice(length);
    for (i, &byte) in mmap[end - length..end].iter().enumerate() {
        header[i] = mem::MaybeUninit::new(!byte);
    }
    let header: Box<[u8]> = unsafe { header.assume_init() };

    let mut reader = BitReader::new(&header);
    let tree = parse_tree(&mut reader).expect("Failed to parse tree");
    let decoded = decode_data(&tree, &mut reader, 0x4C * count as usize);

    let info: &[Info] = unsafe {
        &*ptr::slice_from_raw_parts(decoded.as_ptr() as *const Info, count as usize)
    };

    for i in info {
        let mut extract_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(base.join(i.name()))?;
        let start = i.address as usize;
        let end = start + i.size as usize;
        extract_file.write_all(&mmap[start..end])?
    }

    Ok(())
}

#[inline(always)]
const fn bytes_to_u32_le(bytes: &[u8]) -> u32 {
    (bytes[0] as u32)
        | ((bytes[1] as u32) << 8)
        | ((bytes[2] as u32) << 16)
        | ((bytes[3] as u32) << 24)
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

    fn read_bit(&mut self) -> u8 {
        if self.byte_pos >= self.data.len() {
            return 0;
        }
        let byte = self.data[self.byte_pos];

        let bit = (byte >> (7 - self.bit_pos)) & 1;
        self.bit_pos += 1;
        if self.bit_pos == 8 {
            self.bit_pos = 0;
            self.byte_pos += 1;
        }
        bit
    }

    fn read_bits(&mut self, n: u8) -> Option<u8> {
        let mut val = 0u8;
        for _ in 0..n {
            val <<= 1;
            val |= self.read_bit();
        }
        Some(val)
    }
}

fn parse_tree(reader: &mut BitReader) -> Option<HuffmanNode> {
    let bit = reader.read_bit();
    if bit == 1 {
        let left = parse_tree(reader)?;
        let right = parse_tree(reader)?;
        Some(HuffmanNode::Internal(
            Box::new(left),
            Box::new(right),
        ))
    } else {
        let sym = reader.read_bits(8)?;
        Some(HuffmanNode::Leaf(sym))
    }
}

fn decode_data(
    root: &HuffmanNode,
    reader: &mut BitReader,
    output_len: usize,
) -> Box<[u8]> {
    let mut output = Vec::with_capacity(output_len);
    let mut node = root;

    while output.len() < output_len {
        let bit = reader.read_bit();

        node = match node {
            | HuffmanNode::Internal(left, right) => {
                if bit == 0 {
                    left.as_ref()
                } else {
                    right.as_ref()
                }
            },
            | HuffmanNode::Leaf(sym) => {
                output.push(*sym);
                node = root;

                if bit == 0 {
                    if let HuffmanNode::Internal(left, _) = node {
                        left.as_ref()
                    } else {
                        return output.into_boxed_slice();
                    }
                } else {
                    if let HuffmanNode::Internal(_, right) = node {
                        right.as_ref()
                    } else {
                        return output.into_boxed_slice();
                    }
                }
            },
        }
    }

    if let HuffmanNode::Leaf(sym) = node {
        if output.len() < output_len {
            output.push(*sym);
        }
    }

    output.into_boxed_slice()
}
