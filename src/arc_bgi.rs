use std::fs::File;
use std::io::{self, Write};
use std::mem::{MaybeUninit, transmute};
use std::path::Path;
use std::ptr;

use memmap2::Mmap;

use crate::ptr::{as_u32, as_u32_unaligned};

#[derive(Debug)]
#[repr(C, align(4))]
pub struct Info {
    pub name: [u8; 0x60],
    pub address: u32,
    pub size: u32,
    pub useless: [u8; 24],
}

impl Info {
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

pub fn extract(mmap: Mmap, base: &Path) -> io::Result<()> {
    let count = as_u32(&mmap[12..16]) as usize;
    let end = 16 + count * 0x80;
    let entry = &mmap[16..end];
    let entry = entry.as_ptr() as *const Info;

    let entry = unsafe { &*ptr::slice_from_raw_parts(entry, count) };

    for i in 0..count {
        let name = entry[i].name();
        let address = entry[i].address as usize + end;
        let size = entry[i].size as usize;
        let mut file = File::create(base.join(name))?;
        let content = &mmap[address..address + size];
        match &content[..16] {
            | b"DSC FORMAT 1.00\0" => {
                let mut key = as_u32_unaligned(&content[16..20]);
                let size = as_u32_unaligned(&content[20..24]) as usize;
                let _size = as_u32_unaligned(&content[24..28]);
                let data = &content[32..];
                let mut vec: Vec<HuffmanCode> = Vec::with_capacity(512);
                let mut node: Vec<HuffmanNode> = vec![
                    HuffmanNode {
                        is_parent: true,
                        code: 0,
                        left: 0,
                        right: 0
                    };
                    1023
                ];
                let mut count = 0;
                for i in 0..512u16 {
                    let depth = data[i as usize].wrapping_sub(upkey(&mut key));

                    if depth != 0 {
                        vec.push(HuffmanCode {
                            depth: depth as u16,
                            code: i,
                        });
                        count += 1;
                    }
                }
                unsafe {
                    let ptr = vec.as_mut_ptr();
                    for i in count..512 {
                        ptr.add(i)
                            .write(HuffmanCode { depth: 0, code: 0 });
                    }
                    vec.set_len(512);
                }
                vec[..count].sort();
                create_huffman_tree(&mut node, &vec, count);

                let mut reader = BitReader::new(&data[512..]);
                let mut output = Box::new_uninit_slice(size);

                huffman_decompress(&node, _size, &mut output, &mut reader);
                let content = unsafe { output.assume_init() };
                file.write_all(&content)?;
            },
            | _ => {},
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;
    #[test]
    fn size() {
        assert_eq!(size_of::<Info>(), 0x80);
    }
}

fn upkey(key: &mut u32) -> u8 {
    let k1 = 20021 * (*key & 0xffff);
    let mut k2 = 0x53440000 | (*key >> 16);
    k2 = k2
        .wrapping_mul(20021)
        .wrapping_add(key.wrapping_mul(346));
    k2 = (k2 + (k1 >> 16)) & 0xffff;
    *key = (k2 << 16) + (k1 & 0xffff) + 1;
    k2 as u8
}

#[derive(Debug, Clone)]
pub struct HuffmanNode {
    pub is_parent: bool,
    pub code: u16,
    pub left: usize,
    pub right: usize,
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct HuffmanCode {
    pub code: u16,
    pub depth: u16,
}

impl PartialOrd for HuffmanCode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanCode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.depth.cmp(&other.depth);
        if cmp == std::cmp::Ordering::Equal {
            self.code.cmp(&other.code)
        } else {
            cmp
        }
    }
}

pub fn create_huffman_tree(
    hnodes: &mut [HuffmanNode],
    hcodes: &[HuffmanCode],
    node_count: usize,
) {
    let mut nodes_index = [[0usize; 512]; 2];
    let mut next_node_index = 1;
    let mut depth_nodes = 1;
    let mut depth = 0;
    let mut child_index = 0;

    nodes_index[0][0] = 0;

    let mut n = 0;
    while n < node_count {
        let huffman_nodes_index = child_index;
        child_index ^= 1;

        let mut depth_existed_nodes = 0;
        while n < hcodes.len() && hcodes[n].depth == depth as u16 {
            let idx = nodes_index[huffman_nodes_index][depth_existed_nodes];
            hnodes[idx] = HuffmanNode {
                is_parent: false,
                code: hcodes[n].code,
                left: 0,
                right: 0,
            };
            depth_existed_nodes += 1;
            n += 1;
        }

        let depth_nodes_to_create = if depth_nodes > depth_existed_nodes {
            depth_nodes - depth_existed_nodes
        } else {
            0
        };

        for i in 0..depth_nodes_to_create {
            let left_idx = next_node_index;
            let right_idx = next_node_index + 1;
            next_node_index += 2;

            nodes_index[child_index][i * 2] = left_idx;
            nodes_index[child_index][i * 2 + 1] = right_idx;

            let idx = nodes_index[huffman_nodes_index][depth_existed_nodes + i];
            hnodes[idx] = HuffmanNode {
                is_parent: true,
                code: 0,
                left: left_idx,
                right: right_idx,
            };
        }

        depth += 1;
        depth_nodes = depth_nodes_to_create * 2;
    }
}

struct BitReader<'a> {
    input: &'a [u8],
    byte_pos: usize,
    bit_pos: u8,
}

impl<'a> BitReader<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            byte_pos: 0,
            bit_pos: 0,
        }
    }

    fn get_next_bit(&mut self) -> i32 {
        if self.byte_pos >= self.input.len() {
            return -1;
        }

        let byte = self.input[self.byte_pos];
        let bit = (byte >> (7 - self.bit_pos)) & 1;

        self.bit_pos += 1;
        if self.bit_pos == 8 {
            self.bit_pos = 0;
            self.byte_pos += 1;
        }

        bit as i32
    }

    fn get_bits(&mut self, n: u8) -> i32 {
        if n == 0 || n > 32 {
            return -1;
        }
        let mut val = 0;
        for _ in 0..n {
            let bit = self.get_next_bit();
            if bit == -1 {
                return -1;
            }
            val = (val << 1) | bit;
        }
        val
    }
}

fn huffman_decompress(
    hnodes: &[HuffmanNode],
    dec_count: u32,
    m_output: &mut [MaybeUninit<u8>],
    bit_reader: &mut BitReader,
) {
    let mut dst_ptr = 0usize;

    for _ in 0..dec_count {
        let mut node_index = 0;
        loop {
            let bit = bit_reader.get_next_bit();

            node_index = if bit == 0 {
                hnodes[node_index].left
            } else {
                hnodes[node_index].right
            };
            if !hnodes[node_index].is_parent {
                break;
            }
        }

        let code = hnodes[node_index].code;
        if code >= 256 {
            let offset = bit_reader.get_bits(12);
            if offset == -1 {
                break;
            }
            let offset = (offset as usize) + 2;
            let count = ((code & 0xff) as usize) + 2;

            for i in 0..count {
                m_output[dst_ptr + i] = m_output[dst_ptr - offset + i];
            }
            dst_ptr += count;
        } else {
            m_output[dst_ptr] = MaybeUninit::new(code as u8);
            dst_ptr += 1;
        }
    }
}
