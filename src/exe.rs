#![allow(unused)]
//use chrono::{DateTime, TimeZone, Utc};

use core::mem::{MaybeUninit, transmute};
use std::fs::{self, File};
use std::io::{self, BufWriter, Error, ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::slice;

use encoding_rs::SHIFT_JIS;

//For more details, please refer to
//https://learn.microsoft.com/en-us/windows/win32/debug/pe-format
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ImageDosHeader {
    // DOS .EXE header
    pub e_magic: u16,      // Magic number: "MZ" = 0x5A4D
    pub e_cblp: u16,       // Bytes on last page of file
    pub e_cp: u16,         // Pages in file
    pub e_crlc: u16,       // Relocations
    pub e_cparhdr: u16,    // Size of header in paragraphs
    pub e_minalloc: u16,   // Minimum extra paragraphs needed
    pub e_maxalloc: u16,   // Maximum extra paragraphs needed
    pub e_ss: u16,         // Initial (relative) SS value
    pub e_sp: u16,         // Initial SP value
    pub e_csum: u16,       // Checksum
    pub e_ip: u16,         // Initial IP value
    pub e_cs: u16,         // Initial (relative) CS value
    pub e_lfarlc: u16,     // File address of relocation table
    pub e_ovno: u16,       // Overlay number
    pub e_res: [u16; 4],   // Reserved words
    pub e_oemid: u16,      // OEM identifier (for e_oeminfo)
    pub e_oeminfo: u16,    // OEM information; e_oemid specific
    pub e_res2: [u16; 10], // Reserved words
    pub e_lfanew: i32,     // File address of new exe header (PE header offset)
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ImageNtHeaders32 {
    pub signature: u32, // "PE\0\0"
    pub file_header: ImageFileHeader,
    pub optional_header: ImageOptionalHeader32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ImageFileHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ImageOptionalHeader32 {
    pub magic: u16, //0x010b
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub base_of_data: u32, //pe32 only
    //Optional Header Windows-Specific Fields
    pub image_base: u32,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u32,
    pub size_of_stack_commit: u32,
    pub size_of_heap_reserve: u32,
    pub size_of_heap_commit: u32,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub data_directory: [ImageDataDirectory; 16],
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ImageDataDirectory {
    pub virtual_address: u32,
    pub size: u32,
}

// #[derive(Debug)]
// pub union Misc {
//     pub physical_address: u32,
//     pub virtual_size: u32,
// }

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ImageSectionHeader {
    pub name: [u8; 8],
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub characteristics: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ImageResourceDirectory {
    pub characteristics: u32,
    pub time_date_stamp: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub number_of_named_entries: u16,
    pub number_of_id_entries: u16,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ImageResourceDirectoryEntry {
    pub name_or_id: u32,
    pub offset_to_data: u32,
}

impl ImageResourceDirectoryEntry {
    pub fn is_name_string(&self) -> bool {
        (self.name_or_id & 0x80000000) != 0
    }

    pub fn name_offset(&self) -> u32 {
        self.name_or_id & 0x7FFFFFFF
    }

    pub fn id(&self) -> u16 {
        self.name_or_id as u16
    }

    pub fn is_directory(&self) -> bool {
        (self.offset_to_data & 0x80000000) != 0
    }

    pub fn offset_to_directory(&self) -> u32 {
        self.offset_to_data & 0x7FFFFFFF
    }
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ImageResourceDataEntry {
    pub offset_to_data: u32,
    pub size: u32,
    pub code_page: u32,
    pub reserved: u32,
}

//For more details, please refer to https://github.com/pkuislm/ws2Parse
enum ARG {
    ArgU8 = 0x0,
    ArgI16 = 0x1,
    ArgU16 = 0x2,
    ArgI32 = 0x3,
    ArgU32 = 0x4,
    ArgF32 = 0x5,
    ArgStr1 = 0x6,
    ArgArray = 0x7,
    ArgNull = 0x8,
    ArgStr2 = 0x9,
    ArgStr3 = 0xA,
    //ArgStrUtf8 = 0x0B,
    ArgCallBack = 0xFE,
    ArgEnd = 0xFF,
}

pub fn pattern_search(data: &[u8], pattern: &[u8]) -> Option<usize> {
    let data_len = data.len();
    let pat_len = pattern.len();

    for offset in 0..=data_len.saturating_sub(pat_len) {
        let mut matched = true;

        for i in 0..pat_len {
            let pat_byte = pattern[i];
            let data_byte = data[offset + i];
            if pat_byte != 0x2A && pat_byte != data_byte {
                matched = false;
                break;
            }
        }
        if matched {
            return Some(offset);
        }
    }
    None
}

pub fn get_offset(data: &mut [u8]) -> usize {
    let patterns: &[&[u8]] = &[
        &[0x8B, 0x2C, 0x85, 0x2A, 0x2A, 0x2A, 0x2A, 0x85, 0xED],
        &[0x8B, 0x1C, 0x85, 0x2A, 0x2A, 0x2A, 0x2A, 0x85, 0xDB, 0x75, 0x1F],
        &[0x8B, 0x04, 0x85, 0x2A, 0x2A, 0x2A, 0x2A, 0x89, 0x45, 0xE4],
        &[0x8B, 0x0C, 0x8D, 0x2A, 0x2A, 0x2A, 0x2A, 0x89, 0x4D, 0xDC, 0x85, 0xC9],
        &[0x8B, 0x04, 0x85, 0x2A, 0x2A, 0x2A, 0x2A, 0x89, 0x45, 0xF0, 0x85, 0xC0],
    ];
    for &pattern in patterns {
        if let Some(offset) = pattern_search(data, pattern) {
            return offset + 3;
        }
    }
    0
}

pub fn check(file: &Path, file2: &Path) -> io::Result<()> {
    let mut file = File::open(file)?;
    let mut buffer = [0u8; 0x40];
    file.read_exact(&mut buffer)?;
    let dos_header: ImageDosHeader = unsafe { transmute(buffer) };
    if dos_header.e_magic != 0x5A4D {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "Invalid DOS header: expected e_magic == 0x5A4D ('MZ'), got 0x{:04X}",
                dos_header.e_magic
            ),
        ));
    }
    file.seek(SeekFrom::Start(dos_header.e_lfanew as u64))?;
    let mut buffer = [0u8; 248];
    file.read_exact(&mut buffer)?;
    let image_nt_headers32: ImageNtHeaders32 = unsafe { transmute(buffer) };
    let ImageNtHeaders32 {
        signature,
        file_header,
        optional_header,
    } = image_nt_headers32;
    if signature != 0x4550 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "Invalid PE header: expected signature 0x00004550 ('PE\0\0'), got 0x{:08X}",
                signature
            ),
        ));
    }
    let ImageFileHeader {
        machine,
        number_of_sections,
        time_date_stamp,
        pointer_to_symbol_table,
        number_of_symbols,
        size_of_optional_header,
        characteristics,
    } = file_header;
    let ImageOptionalHeader32 {
        magic,
        major_linker_version,
        minor_linker_version,
        size_of_code,
        size_of_initialized_data,
        size_of_uninitialized_data,
        address_of_entry_point,
        base_of_code,
        base_of_data,
        image_base,
        section_alignment,
        file_alignment,
        major_operating_system_version,
        minor_operating_system_version,
        major_image_version,
        minor_image_version,
        major_subsystem_version,
        minor_subsystem_version,
        win32_version_value,
        size_of_image,
        size_of_headers,
        checksum,
        subsystem,
        dll_characteristics,
        size_of_stack_reserve,
        size_of_stack_commit,
        size_of_heap_reserve,
        size_of_heap_commit,
        loader_flags,
        number_of_rva_and_sizes,
        data_directory,
    } = optional_header;

    //let datetime: DateTime<Utc> = Utc.timestamp_opt(time_date_stamp as i64, 0).unwrap();
    //println!("time date stamp is: {datetime}");
    if size_of_optional_header != 224 {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            format!(
                "Invalid SizeOfOptionalHeader: expected 224 (0xE0) for PE32, got (0x{:04X})",
                size_of_optional_header
            ),
        ));
    }

    debug_assert_eq!(size_of_optional_header, 224);
    if magic != 0x10b {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "Unsupported Optional Header Magic: expected 0x10B (PE32), got 0x{:X}",
                magic
            ),
        ));
    }
    debug_assert_eq!(magic, 0x10b);

    let mut buffer: Box<[MaybeUninit<u8>]> =
        Box::new_uninit_slice(40 * number_of_sections as usize);
    let bytes = unsafe {
        slice::from_raw_parts_mut(
            buffer.as_mut_ptr() as *mut u8,
            40 * number_of_sections as usize,
        )
    };
    file.read_exact(bytes)?;

    //This is incorrect, but I want to do it this way
    let ptr = Box::into_raw(buffer) as *mut ImageSectionHeader as *mut [ImageSectionHeader; 4];
    let section_header: Box<[ImageSectionHeader; 4]> = unsafe { Box::from_raw(ptr) };
    let [text, rdata, data, rsrc] = *section_header;

    file.seek(SeekFrom::Start(text.pointer_to_raw_data as u64))?;
    let len = text.size_of_raw_data as usize;

    let mut buffer: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(len);
    let text_byte = unsafe { slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, len) };
    file.read_exact(text_byte)?;
    let offset = get_offset(text_byte);
    if offset == 0 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "No matching data was found!",
        ));
    }
    let bytes: [u8; 4] = text_byte[offset..offset + 4].try_into().unwrap();
    let mut address = u32::from_le_bytes(bytes);

    //This is unnecessary and useless, just to verify my hypothesis
    debug_assert_eq!(image_base, 0x400000);

    //Checking which section a virtual address belongs to is important for correct processing,
    //but right now I do not want to carry out this step.
    //section.VirtualAddress <= rva < section.VirtualAddress + section.VirtualSize
    //rva=va-image_base
    debug_assert!(
        rdata.virtual_address <= address - image_base
            && address - image_base < rdata.virtual_address + rdata.virtual_size
    );
    //file_offset = section.PointerToRawData + (rva - section.VirtualAddress)
    let address = address - image_base - rdata.virtual_address + rdata.pointer_to_raw_data;

    file.seek(SeekFrom::Start(address as u64))?;
    let mut buffer: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(1024);
    let raw_bytes: &mut [u8] =
        unsafe { slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, 1024) };
    file.read_exact(raw_bytes)?;
    let instruction_slice: &mut [u32] =
        unsafe { std::slice::from_raw_parts_mut(raw_bytes.as_mut_ptr() as *mut u32, 256) };

    //The smallest in instruction_slice
    let mut min = 0xffffffff;

    //The largest in instruction_slice
    let mut max = 0;

    let datava = data.virtual_address;
    let datap = data.pointer_to_raw_data;

    #[cfg(debug_assertions)]
    let datavs = data.virtual_size;

    for i in instruction_slice.iter_mut() {
        if *i == 0 {
            continue;
        }
        //important ?
        #[cfg(debug_assertions)]
        debug_assert!(datava <= *i - image_base && *i - image_base < datava + datavs);
        *i = *i - image_base - datava + datap;
        if *i > max {
            max = *i;
        }
        if *i < min {
            min = *i - 1;
        }
    }

    for i in instruction_slice.iter_mut() {
        if *i == 0 {
            continue;
        }
        *i -= min;
    }

    //bad if less than 10, 10 to 25 are OK, and unnecessary if more than 30.
    let len_maybe = (max - min + 20) as usize;

    //This is unnecessary and useless, just to verify my hypothesis
    #[cfg(debug_assertions)]
    {
        let zero_count = instruction_slice
            .iter()
            .filter(|&&x| x == 0)
            .count();
        assert_eq!(zero_count, 79);
    }
    file.seek(SeekFrom::Start(min as u64))?;
    let mut buffer: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(len_maybe);
    let raw_bytes: &mut [u8] =
        unsafe { slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, len_maybe) };
    file.read_exact(raw_bytes)?;
    let arg_slice: &mut [u8] =
        unsafe { std::slice::from_raw_parts_mut(raw_bytes.as_mut_ptr() as *mut u8, len_maybe) };

    //
    //
    //
    //
    //
    let mut file = File::open(file2)?;
    let file_len = file.metadata()?.len() as usize;

    let mut buffer: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(file_len);
    let initialized_slice = unsafe {
        let buf_ptr = buffer.as_mut_ptr() as *mut u8;
        let slice = slice::from_raw_parts_mut(buf_ptr, file_len);
        file.read_exact(slice)?;
        let initialized = buffer.assume_init();
        initialized
    };

    let json = File::create("output.json")?;
    let mut writer = BufWriter::new(json);
    //
    write!(writer, "{{\n")?;
    let mut index = 0;
    loop {
        if index >= file_len {
            break;
        }
        let code = initialized_slice[index];

        let mut arg_index = instruction_slice[code as usize] as usize;
        if arg_index == 0 {
            index += 1;
            continue;
        }
        write!(writer, "  \"instruction#{:02X}\": [\n", code)?;
        index += 1;
        loop {
            match arg_slice[arg_index] {
                | 0x00 => {
                    //u8
                    write!(
                        writer,
                        "    [\"0x00\",\"{:02X}\"],\n",
                        initialized_slice[index]
                    )?;
                    arg_index += 1;
                    index += 1;
                },
                | 0x01 => {
                    //i16
                    let bytes: [u8; 2] = initialized_slice[index..index + 2]
                        .try_into()
                        .unwrap();
                    write!(
                        writer,
                        "    [\"0x01\",\"{:04X}\"],\n",
                        i16::from_le_bytes(bytes)
                    )?;
                    arg_index += 1;
                    index += 2;
                },
                | 0x02 => {
                    //u16
                    let bytes: [u8; 2] = initialized_slice[index..index + 2]
                        .try_into()
                        .unwrap();
                    write!(
                        writer,
                        "    [\"0x02\",\"{:04X}\"],\n",
                        u16::from_le_bytes(bytes)
                    )?;
                    arg_index += 1;
                    index += 2;
                },
                | 0x03 => {
                    //i32
                    let bytes: [u8; 4] = initialized_slice[index..index + 4]
                        .try_into()
                        .unwrap();
                    write!(
                        writer,
                        "    [\"0x03\",\"{:08X}\"],\n",
                        i32::from_le_bytes(bytes)
                    )?;
                    arg_index += 1;
                    index += 4;
                },
                | 0x04 => {
                    //u32
                    let bytes: [u8; 4] = initialized_slice[index..index + 4]
                        .try_into()
                        .unwrap();
                    write!(
                        writer,
                        "    [\"0x04\",\"{:08X}\"],\n",
                        u32::from_le_bytes(bytes)
                    )?;
                    arg_index += 1;
                    index += 4;
                },
                | 0x05 => {
                    //f32
                    let bytes: [u8; 4] = initialized_slice[index..index + 4]
                        .try_into()
                        .unwrap();
                    write!(
                        writer,
                        "    [\"0x05\",\"{:08X}\"],\n",
                        u32::from_le_bytes(bytes)
                    )?;
                    arg_index += 1;
                    index += 4;
                },
                | 0x06 | 0x09 | 0x0A => {
                    let start = index;
                    loop {
                        if initialized_slice[index] == 0x00 {
                            break;
                        }
                        index += 1;
                    }
                    let (data, ..) = SHIFT_JIS.decode(&initialized_slice[start..index]);
                    write!(
                        writer,
                        "    [\"0x{:02X}\",\"{data}\"],\n",
                        arg_slice[arg_index]
                    )?;
                    arg_index += 1;
                },
                | 0x07 => {
                    write!(writer, "    [\"0x07\",{}],\n", initialized_slice[index])?;
                    index += 1;
                    arg_index += 1;
                    for i in 0..initialized_slice[index] {
                        match arg_slice[arg_index] {
                            | 0x00 => {
                                write!(
                                    writer,
                                    "    [\"0x00\",\"{:02X}\"],\n",
                                    initialized_slice[index]
                                )?;
                                index += 1;
                            },
                            | 0x01 => {
                                let bytes: [u8; 2] = initialized_slice[index..index + 2]
                                    .try_into()
                                    .unwrap();
                                write!(
                                    writer,
                                    "    [\"0x01\",\"{:04X}\"],\n",
                                    i16::from_le_bytes(bytes)
                                )?;
                                index += 2;
                            },
                            | 0x02 => {
                                let bytes: [u8; 2] = initialized_slice[index..index + 2]
                                    .try_into()
                                    .unwrap();
                                write!(
                                    writer,
                                    "    [\"0x02\",\"{:04X}\"],\n",
                                    u16::from_le_bytes(bytes)
                                )?;
                                index += 2;
                            },
                            | 0x03 => {
                                let bytes: [u8; 4] = initialized_slice[index..index + 4]
                                    .try_into()
                                    .unwrap();
                                write!(
                                    writer,
                                    "    [\"0x03\",\"{:08X}\"],\n",
                                    i32::from_le_bytes(bytes)
                                )?;
                                index += 4;
                            },
                            | 0x04 => {
                                let bytes: [u8; 4] = initialized_slice[index..index + 4]
                                    .try_into()
                                    .unwrap();
                                write!(
                                    writer,
                                    "    [\"0x03\",\"{:08X}\"],\n",
                                    u32::from_le_bytes(bytes)
                                )?;
                                index += 4;
                            },
                            | 0x05 => {
                                let bytes: [u8; 4] = initialized_slice[index..index + 4]
                                    .try_into()
                                    .unwrap();
                                write!(
                                    writer,
                                    "    [\"0x03\",\"{:08X}\"],\n",
                                    u32::from_le_bytes(bytes)
                                )?;
                                index += 4;
                            },
                            | 0x06 | 0x09 | 0x0A => {
                                let start = index;
                                loop {
                                    if initialized_slice[index] == 0x00 {
                                        break;
                                    }
                                    index += 1;
                                }
                                let (data, ..) = SHIFT_JIS.decode(&initialized_slice[start..index]);
                                write!(
                                    writer,
                                    "    [\"0x{:02X}\",\"{data}\"],\n",
                                    arg_slice[arg_index]
                                )?;
                            },
                            | 0x08 => {
                                write!(writer, "    [\"0x08\",\"0x00\"],\n")?;
                                index += 1;
                            },
                            | 0xFE => {
                                write!(writer, "    [\"0xFE\",\"\"],\n")?;
                            },
                            | _ => {},
                        }
                    }
                },
                | 0x08 => {
                    write!(writer, "    [\"0x08\",\"0x00\"],\n")?;
                    index += 1;
                    arg_index += 1;
                },
                | 0xFE => {
                    write!(writer, "    [\"0xFE\",\"\"],\n")?;
                    arg_index += 1;
                },
                | 0xFF => {
                    write!(writer, "    [\"0xFF\",\"END\"]\n")?;
                    break;
                },
                | _ => {},
            }
        }
        write!(writer, "  ],\n")?;
    }
    write!(writer, "}}")?;
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;
    #[test]
    fn size() {
        assert_eq!(size_of::<ImageDosHeader>(), 64);
        assert_eq!(size_of::<ImageNtHeaders32>(), 248);
        assert_eq!(size_of::<ImageFileHeader>(), 20);
        assert_eq!(size_of::<ImageOptionalHeader32>(), 224);
        assert_eq!(size_of::<ImageDataDirectory>(), 8);
        assert_eq!(size_of::<ImageSectionHeader>(), 40);
    }
}
