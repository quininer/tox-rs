extern crate tox;
extern crate rustc_serialize;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env::args;
use rustc_serialize::hex::ToHex;
use tox::core::{
    ToxOptions, Status
};


fn fileread<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut data = Vec::new();
    File::open(path).unwrap()
        .read_to_end(&mut data).ok();
    data
}

fn main() {
    let im = ToxOptions::new()
        .from(&fileread(args().nth(1).expect("listfriend PROFILE.tox")))
        .generate().expect("Parser Error.");

    println!(r#"YOU
name: {}
ToxID: {}
"#,
        String::from_utf8_lossy(&im.name().unwrap()),
        im.address().unwrap().out().to_hex()
    );

    println!("Friends");
    for f in im.list_friend() {
        println!("ToxPK: {}", f.publickey().unwrap().raw.to_hex())
    }
}
