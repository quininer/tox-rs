#![feature(stmt_expr_attributes)]

extern crate ffigen;

use std::env::var;
use std::fs::File;
use std::io::Write;
use ffigen::GenOptions;

const CLANG_INCLUDE_PATH: &'static str = "/usr/lib/clang/3.7.1/include/";
const TOX_INCLUDE_PATH: &'static str = "/usr/include/tox/";


macro_rules! gen {
    ( $l:expr, [ $( $h:expr ),* ], $o:expr ) => {{
        let data = GenOptions::new()
            .arg(&format!("-I{}", var("CLANG_INCLUDE_PATH").unwrap_or(CLANG_INCLUDE_PATH.into())))
        $(
            .header(&format!("{}{}", var("TOX_INCLUDE_PATH").unwrap_or(TOX_INCLUDE_PATH.into()), $h))
        )*
            .link($l)
            .gen();

        File::create(format!("src/{}", $o)).unwrap()
            .write(&data).unwrap();
    }};
}


fn main() {
    #[cfg(feature = "groupchat")]
    gen!("toxcore", ["tox.h", "tox_old.h"], "core/ffi.rs");

    #[cfg(feature = "newgroupchat")]
    gen!("toxcore", ["tox.h"], "core/ffi.rs");

    gen!("toxav", ["toxav.h"], "av/ffi.rs");
    gen!("toxencryptsave", ["toxencryptsave.h"], "encryptsave/ffi.rs");
}
