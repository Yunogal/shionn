use std::cmp::Ordering;
use std::io::Result;

#[derive(Debug)]
#[repr(C)]
pub struct DSC {
    pub signature: [u8; 16], //'DSC FORMAT 1.00\0'
    pub key: u32,
    pub zsize: u32,
    pub size: u32,
    pub zero: u32,
}
#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<DSC>(), 4);
    assert_eq!(size_of::<DSC>(), 0x20);
}

fn update(key: &mut u32) -> u8 {
    let k1 = 20021 * (*key & 0xffff);
    let mut k2 = 0x53440000 | (*key >> 16);
    k2 = k2 * 20021 + *key * 346;
    k2 = (k2 + (k1 >> 16)) & 0xffff;
    *key = (k2 << 16) + (k1 & 0xffff) + 1;
    k2 as u8
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Leaf {
    pub symbol: u16,
    pub depth: u16,
}
impl PartialOrd for Leaf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Leaf {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = self.depth.cmp(&other.depth);
        if cmp == Ordering::Equal {
            self.symbol.cmp(&other.symbol)
        } else {
            cmp
        }
    }
}
pub fn parse(content: &[u8]) -> Result<Vec<u8>> {
    // let file = File::open("dsc")?;
    // let mmap = unsafe { memmap2::MmapOptions::new().map(&file)? };
    // let content = mmap.as_ref();
    let ptr = content.as_ptr() as *const DSC;
    let dsc = unsafe { &*ptr };
    let mut key = dsc.key;
    let size = dsc.size as usize;
    let mut leaf: Vec<Leaf> = Vec::with_capacity(512);
    let mut codes: Box<[u16; 512]> = Box::new([0; 512]);
    let mut lengths: Box<[u16; 512]> = Box::new([0; 512]);
    let data = &content[32..];
    for i in 0..512 {
        let depth = data[i] - update(&mut key);
        if depth != 0 {
            leaf.push(Leaf {
                symbol: i as u16,
                depth: depth as u16,
            });
        }
    }
    leaf.sort();
    let max_bits = leaf[leaf.len() - 1].depth;
    let mut code: u16 = 0;
    let mut len = 0;
    for Leaf { symbol, depth } in leaf {
        lengths[symbol as usize] = depth;

        if depth > len {
            code <<= depth - len;
            len = depth;
        }
        let start = code << (max_bits - depth);
        let end = (code + 1) << (max_bits - depth);

        for idx in start..end {
            codes[idx as usize] = symbol;
        }
        code += 1;
    }
    let mut reader = BitReader::new(&data[512..]);
    let output_len = size;
    let max_bits = max_bits as u8;
    let mut output = Vec::with_capacity(output_len);

    while output.len() < output_len {
        let bits = match reader.peek_bits(max_bits) {
            | Some(b) => b,
            | None => break,
        };

        let symbol = codes[bits as usize];
        let depth = lengths[symbol as usize];
        reader.drop_bits(depth as u8);

        if symbol < 256 {
            output.push(symbol as u8);
        } else {
            let offset = reader.read_bits(12).unwrap() as usize + 2;
            let count = (symbol & 0xff) as usize + 2;
            let start = output.len() - offset;
            for i in 0..count {
                let val = output[start + i];
                output.push(val);
            }
        }
    }
    Ok(output)
}

struct BitReader<'a> {
    data: &'a [u8],
    byte_pos: usize,
    bit_buffer: u16,
    bit_count: u8,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            byte_pos: 0,
            bit_buffer: 0,
            bit_count: 0,
        }
    }

    fn fill_bits(&mut self, n: u8) {
        while self.bit_count < n && self.byte_pos < self.data.len() {
            self.bit_buffer <<= 8;
            self.bit_buffer |= self.data[self.byte_pos] as u16;
            self.bit_count += 8;
            self.byte_pos += 1;
        }
    }

    fn read_bits(&mut self, n: u8) -> Option<u16> {
        self.fill_bits(n);
        if self.bit_count < n {
            return None;
        }
        let shift = self.bit_count - n;
        let bits = (self.bit_buffer >> shift) & ((1 << n) - 1);
        self.bit_count -= n;
        self.bit_buffer &= (1 << self.bit_count) - 1;
        Some(bits)
    }

    fn peek_bits(&mut self, n: u8) -> Option<u16> {
        self.fill_bits(n);
        if self.bit_count < n {
            return None;
        }
        let shift = self.bit_count - n;
        Some((self.bit_buffer >> shift) & ((1 << n) - 1))
    }

    fn drop_bits(&mut self, n: u8) {
        self.bit_count -= n;
        self.bit_buffer &= (1 << self.bit_count) - 1;
    }
}
