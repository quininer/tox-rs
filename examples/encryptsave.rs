extern crate tox;

use std::env::args;
use std::fs::File;
use std::path::Path;
use std::io::{ Read, Write };
use tox::encryptsave;


fn fileread<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut data = Vec::new();
    File::open(path).unwrap()
        .read_to_end(&mut data).ok();
    data
}

fn main() {
    let mut argv = args().skip(1);

    let result = match argv.next()
        .expect("encryptsave (en | de) KEY INPUT OUT")
        .as_ref()
    {
        "en" => encryptsave::pass_encrypt(
            &argv.next().unwrap().as_bytes(),
            &fileread(argv.next().unwrap())
        ).unwrap(),
        "de" => encryptsave::pass_decrypt(
            &argv.next().unwrap().as_bytes(),
            &fileread(argv.next().unwrap())
        ).unwrap(),
        "is" => {
            return println!("{}", encryptsave::is_encrypted(
                &fileread(argv.next().unwrap())
            ));
        },
        _ => panic!("wtf")
    };

    File::create(argv.next().unwrap()).unwrap()
        .write(&result).ok();
}
