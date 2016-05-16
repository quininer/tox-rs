#![feature(stmt_expr_attributes)]

#[macro_use] extern crate ffigen;

fn main() {
    #[cfg(feature = "groupchat")]
    gen!("toxcore", ["tox/tox.h", "tox/tox_old.h"] -> "src/core/ffi.rs");

    #[cfg(feature = "new-groupchat")]
    gen!("toxcore", ["tox/tox.h"] -> "src/core/ffi.rs");

    gen!("toxav", ["tox/toxav.h"] -> "src/av/ffi.rs");
    gen!("toxencryptsave", ["tox/toxencryptsave.h"] -> "src/encryptsave/ffi.rs");
}
