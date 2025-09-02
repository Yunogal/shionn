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
    }
}
null!(null);

macro_rules! truncation {
    ($( $ident:ident ),*) => {
        $(
        pub struct $ident;
        impl Filter for $ident {
            fn filter(&self, content: &mut [u8], hash: u32) {
                let key=hash as u8;
                for i in content{
                    *i^=key;
                }
            }
        }
    )*
    }
}
truncation!(truncation);

macro_rules! xor {
    ($( $ident:ident => $key:literal),*) => {
        $(
        pub struct $ident;
        impl Filter for $ident {
            fn filter(&self, content: &mut [u8], _hash: u32) {
                for i in content{
                    *i^=$key;
                }
            }
        }
    )*
    }
}

// p137 ASa Project

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
    }
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

// madosoft p3312
asaproject!(v12288);

pub struct v14888;

impl Filter for v14888 {
    fn filter(&self, content: &mut [u8], hash: u32) {
        madosoft_filter(content, hash, 0xF2FAA99F);
    }
}

fn madosoft_filter(content: &mut [u8], mut hash: u32, num: u32) {
    hash ^= num;
    hash &= 0x7fffffff;
    hash = (hash << 31) | hash;
    let mut key: [u8; 31] = [0; 31];
    for i in 0..31 {
        key[i] = hash as u8;
        hash = (hash & 0xfffffffe) << 23 | hash >> 8;
    }
    for (i, byte) in content.iter_mut().enumerate() {
        *byte ^= key[i % 31];
    }
}

//pub struct v17823;
//v20524

// p1612 Hulotte

macro_rules! Hulotte {
    ($( $ident:ident ),*) => {
        $(
        pub struct $ident;
        impl Filter for $ident {
        fn filter(&self, content: &mut [u8], hash: u32) {
            let key = hash as u8 ^ 0xCD;
             for i in content {
                *i ^= key;
            }
        }
    }
    )*
    }
}
Hulotte!(v5209, v13260, v15437);
truncation!(v17790);
xor!(v19769=>0xCD,v23388=>0x35,v26989=>0x95,v29187=>0xF7,v31002=>0x0E,v44098=>0x3C);

// p4488 NEKO WORKs

// pub struct v15538;
// impl Filter for v15538 {
//     fn filter(&self, content: &mut [u8], hash: u32) {
//         let mut key = hash ^ 0x1548E29C;
//         key = hash ^ (key >> 8) ^ (key >> 16) ^ (key >> 24) ^ 0x9C;
//         if (key & 0xFF) == 0 {
//             key = 215;
//         }
//         let key = key as u8;
//         for i in content {
//             *i ^= key;
//         }
//     }
// }
// pub struct v17763;
// impl Filter for v17763 {
//     fn filter(&self, content: &mut [u8], hash: u32) {
//         let mut key = hash ^ 0x1548E29C;
//         key = hash ^ (key >> 8) ^ (key >> 16) ^ (key >> 24);
//         if (key & 0xFF) == 0 {
//             key = 215;
//         }
//         let key = key as u8;
//         for i in content {
//             *i ^= key;
//         }
//     }
// }
