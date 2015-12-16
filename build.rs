extern crate bindgen;

use std::fs::File;
use std::io::{ Read, Write };
use bindgen::builder;

const CLANG_INCLUDE_PATH: &'static str = "/usr/lib/clang/3.7.0/include/";
const TOX_INCLUDE_PATH: &'static str = "/usr/include/tox/";
const HEAD: &'static [u8] = b"#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type int16_t = i16;
pub type int32_t = i32;

";


fn gen(l: &str, h: &str, o: &str) {
    builder()
        .header(format!("-I{}", CLANG_INCLUDE_PATH))
        .link(l)
        .match_pat(h)
        .header(format!("{}{}", TOX_INCLUDE_PATH, h))
        .generate().unwrap()
        .write_to_file(format!("src/{}", o))
        .map(|_| {
            let mut data = Vec::new();
            File::open(format!("src/{}", o)).unwrap()
                .read_to_end(&mut data).unwrap();
            File::create(format!("src/{}", o)).unwrap()
                .write(&vec![HEAD, &data].concat()).unwrap();
        }).ok();
}

fn main() {
    gen("toxcore", "tox.h", "core/ffi.rs");
    gen("toxav", "toxav.h", "av/ffi.rs");
    gen("toxencryptsave", "toxencryptsave.h", "encryptsave/ffi.rs");
}
