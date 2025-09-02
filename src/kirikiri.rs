#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub trait Filter {
    #[inline]
    fn filter(&self, _content: &mut [u8], _hash: u32) {}
}

macro_rules! null {
    ($( $ident:ident ),*) => {
        $(
        pub struct $ident;
        impl Filter for $ident {}
    )*
    };
}
null!(null);

// p137

macro_rules! asaproject {
    ($( $ident:ident ),*) => {
        $(
        pub struct $ident;
        impl Filter for $ident {
        fn filter(&self, content: &mut [u8], hash: u32) {
        asaproject_filter(content, hash);
    }
}
    )*
    };
}
null!(v776, v814, v3671, v6714);

pub struct v11300;

asaproject!(
    v16339, v19444, v22045, v25366, v26419, v26418, v28633, v30238, v30239, v32811,
    v36615, v36616, v44195, v49227, v52702, v57107
);

impl Filter for v11300 {
    fn filter(&self, content: &mut [u8], hash: u32) {
        v11300_filter(content, hash);
    }
}

fn v11300_filter(content: &mut [u8], hash: u32) {
    let key = v11300_asaproject_generate_key(hash);
    for (i, byte) in content.iter_mut().enumerate() {
        *byte ^= key[i % 31];
        *byte += v11300_key[i % 0x3D];
    }
}

fn v11300_asaproject_generate_key(hash: u32) -> [u8; 31] {
    let mut k: u32 = hash & 0x7fffffff;

    let mut t: [u8; 31] = [0; 31];

    for i in 0..31 {
        t[i] = (k & 0xFF) as u8;
        k = (k << 23) | (k >> 8);
    }
    t
}
const v11300_key: [u8; 0x3D] = [
    0x2E, 0x30, 0x88, 0xC9, 0xEC, 0x29, 0x90, 0xDE, 0x05, 0x06, 0x31, 0x99, 0x3D,
    0x05, 0xD2, 0xBB, 0xC0, 0x20, 0x26, 0xB3, 0xA7, 0x40, 0x7A, 0x17, 0x18, 0xC4,
    0x64, 0xF6, 0x14, 0x48, 0xEF, 0x02, 0x83, 0x98, 0xCC, 0x9E, 0x02, 0xE9, 0x5D,
    0x60, 0x10, 0x93, 0xD9, 0x53, 0x20, 0xBD, 0x0B, 0x0C, 0x62, 0x32, 0x7B, 0x0A,
    0xA4, 0x77, 0x81, 0x41, 0x4C, 0x66, 0x4F, 0x81, 0xF4,
];

fn asaproject_filter(content: &mut [u8], hash: u32) {
    let key = asaproject_generate_key(hash);
    for (i, byte) in content.iter_mut().enumerate() {
        *byte ^= key[i % 31];
    }
}
fn asaproject_generate_key(mut hash: u32) -> [u8; 31] {
    hash &= 0x7fffffff;
    hash = (hash << 31) | hash;

    let mut key: [u8; 31] = [0; 31];

    for i in 0..31 {
        key[i] = hash as u8;
        hash = (hash & 0xfffffffe) << 23 | hash >> 8;
    }
    key
}
