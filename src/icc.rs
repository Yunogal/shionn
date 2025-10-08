/// International Color Consortium
/// https://www.color.org/specification/ICC.1-2022-05.pdf

// (33/126)
pub struct ProfileHeader {
    pub profile_size: u32,
    pub preferred_cmm_type: u32,
    pub profile_version_number: [u8; 4],
    pub profile_or_Device_class: u32,
    pub Colour_space_of_data: u32,
    pub PCS: u32,
    pub time: [u16; 6], // year-month-day-hour-minute-second
}
fn main() {}
// C:\Windows\System32\spool\drivers\color
// Image Color Management
// WCS Gamut Mapping Profile (.gmmp)
// WCS Device Profile (.cdmp)
// https://learn.microsoft.com/en-us/windows/win32/wcs/wcs-gamut-map-model-profile-schema-and-algorithms
