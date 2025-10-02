//For more details, please refer to
//https://learn.microsoft.com/en-us/windows/win32/debug/pe-format

//use chrono::{DateTime, TimeZone, Utc};
#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
pub struct ImageNtHeaders32 {
    pub signature: u32, // "PE\0\0"
    pub file_header: ImageFileHeader,
    pub optional_header: ImageOptionalHeader32,
}

#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
pub struct ImageResourceDirectory {
    pub characteristics: u32,
    pub time_date_stamp: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub number_of_named_entries: u16,
    pub number_of_id_entries: u16,
}

#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
pub struct ImageResourceDataEntry {
    pub offset_to_data: u32,
    pub size: u32,
    pub code_page: u32,
    pub reserved: u32,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<ImageDosHeader>(), 4);
    assert_eq!(size_of::<ImageDosHeader>(), 64);
    assert_eq!(align_of::<ImageNtHeaders32>(), 4);
    assert_eq!(size_of::<ImageNtHeaders32>(), 248);
    assert_eq!(align_of::<ImageFileHeader>(), 4);
    assert_eq!(size_of::<ImageFileHeader>(), 20);
    assert_eq!(align_of::<ImageOptionalHeader32>(), 4);
    assert_eq!(size_of::<ImageOptionalHeader32>(), 224);
    assert_eq!(align_of::<ImageDataDirectory>(), 4);
    assert_eq!(size_of::<ImageDataDirectory>(), 8);
    assert_eq!(align_of::<ImageSectionHeader>(), 4);
    assert_eq!(size_of::<ImageSectionHeader>(), 40);
}
