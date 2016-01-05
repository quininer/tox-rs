extern crate ffigen;

use std::fs::File;
use std::io::Write;
use ffigen::GenOptions;

const CLANG_INCLUDE_PATH: &'static str = "/usr/lib/clang/3.7.0/include/";
const TOX_INCLUDE_PATH: &'static str = "/usr/include/tox/";


fn gen(l: &str, h: &str, o: &str) {
    let data = GenOptions::new()
        .arg(&format!("-I{}", CLANG_INCLUDE_PATH))
        .link(&format!("{}{}", TOX_INCLUDE_PATH, h))
        .pat(l)
        .gen();

    File::create(format!("src/{}", o)).unwrap()
        .write(&data).unwrap();
}

fn main() {
    gen("toxcore", "tox.h", "core/ffi.rs");
    gen("toxav", "toxav.h", "av/ffi.rs");
    gen("toxencryptsave", "toxencryptsave.h", "encryptsave/ffi.rs");
}
