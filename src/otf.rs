// https://fonts.google.com

// https://online-fonts.com

// https://fontforge.org

// https://learn.microsoft.com/en-us/typography/opentype/spec/otff

// All OpenType fonts use big-endian (network) byte order

use std::io::Result;

#[repr(C)]
pub struct OTF {
    pub sfnt_version: u32, // 0x00010000 or 0x4F54544F ('OTTO')
    pub num_tables: u16,   // Number of tables.
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
}

#[repr(C)]
pub struct Table {
    pub tag: [u8; 4],  // Table identifier.
    pub checksum: u32, // Checksum for this table.
    pub offset: u32,   // Offset from beginning of font file.
    pub length: u32,   // Length of this table.
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<OTF>(), 4);
    assert_eq!(size_of::<OTF>(), 12);
    assert_eq!(align_of::<Table>(), 4);
    assert_eq!(size_of::<Table>(), 16);
}

// https://learn.microsoft.com/en-us/typography/opentype/spec/otff#calculating-checksums
pub fn clac_table_checksum(mut table: *const u32, length: u32) -> u32 {
    let mut sum = 0;
    let end_ptr = unsafe { table.add(((length as usize + 3) & !3) / 4) };
    unsafe {
        while table < end_ptr {
            sum += (*table).swap_bytes();
            table = table.add(1);
        }
    }
    sum
}

pub fn parse(content: &[u8]) -> Result<()> {
    let ptr = content.as_ptr();
    let OTF {
        sfnt_version: _,
        num_tables,
        search_range: _,
        entry_selector: _,
        range_shift: _,
    } = unsafe { ptr.cast::<OTF>().read() };
    let mut table_ptr: *const Table = unsafe { ptr.add(12).cast() };
    for _ in 0..num_tables.swap_bytes() {
        let Table {
            tag: _,
            checksum,
            offset,
            length,
        } = unsafe { table_ptr.read() };
        let _checksum = checksum.swap_bytes();
        let offset = offset.swap_bytes() as usize;
        let length = length.swap_bytes();
        unsafe { table_ptr = table_ptr.add(1) };
        let table = unsafe { ptr.add(offset) };
        let _check = clac_table_checksum(table.cast(), length);
    }

    Ok(())
}

#[test]
#[ignore]
fn main() -> Result<()> {
    use memmap2::MmapOptions;
    use std::fs::File;
    let file = File::open(r".otf")?;
    let mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    parse(&mmap[..])?;
    Ok(())
}
