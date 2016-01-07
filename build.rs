extern crate ffigen;

use std::fs::File;
use std::io::Write;
use ffigen::GenOptions;

const CLANG_INCLUDE_PATH: &'static str = "/usr/lib/clang/3.7.0/include/";
const TOX_INCLUDE_PATH: &'static str = "/usr/include/tox/";


macro_rules! gen {
    ( $l:expr, [ $( $h:expr ),* ], $o:expr ) => {{
        let data = GenOptions::new()
            .arg(&format!("-I{}", CLANG_INCLUDE_PATH))
        $(
            .header(&format!("{}{}", TOX_INCLUDE_PATH, $h))
        )*
            .link($l)
            .gen();

        File::create(format!("src/{}", $o)).unwrap()
            .write(&data).unwrap();
    }};
}


fn main() {
    gen!("toxcore", ["tox.h", "tox_old.h"], "core/ffi.rs");
    // gen!("toxcore", ["tox.h"], "core/ffi.rs");
    gen!("toxav", ["toxav.h"], "av/ffi.rs");
    gen!("toxencryptsave", ["toxencryptsave.h"], "encryptsave/ffi.rs");
}
