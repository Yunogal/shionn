use chrono::DateTime;
use std::convert::TryInto;

fn parse_hex_bytes(hex_str: &str) -> Vec<u8> {
    hex_str
        .split_whitespace()
        .map(|h| u8::from_str_radix(h, 16).expect("invalid hex"))
        .collect()
}

fn bytes_to_time(hex_str: &str) -> (u32, String) {
    let bytes = parse_hex_bytes(hex_str);

    let ts = u32::from_le_bytes(bytes[..4].try_into().unwrap());

    let dt = DateTime::from_timestamp(ts as i64, 0).unwrap();
    let datetime_str = dt.format("%Y-%m-%d %H:%M:%S").to_string();

    (ts, datetime_str)
}

fn main() {
    let data = "00 00 00 00";
    let (ts, dt) = bytes_to_time(data);
    println!("Integer value: {}", ts);
    println!("utc time: {}", dt);
}
