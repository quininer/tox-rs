extern crate clap;
extern crate tox;

use std::io::stdout;
use std::fs::File;
use std::path::Path;
use std::io::{ Read, Write };
use clap::{ App, Arg, SubCommand };
use tox::encryptsave;


fn read<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut data = Vec::new();
    File::open(path).unwrap().read_to_end(&mut data).ok();
    data
}

fn write<P: AsRef<Path>>(path: P, data: &[u8]) {
    File::create(path).unwrap().write(data).ok();
}

fn main() {
    let app = App::new("EncryptSave")
        .about("Tox-rs examples - encryptsave.")
        .args(&[
            Arg::from_usage("-p, --passwd [passphrase] 'passphrase.'").global(true),
            Arg::from_usage("-o, --output [file] 'out file.'").global(true)
        ])
        .subcommands(vec![
            SubCommand::with_name("en")
                .about("Encryption")
                .arg(Arg::from_usage("<input> 'input file.'")),
            SubCommand::with_name("de")
                .about("Decryption")
                .arg(Arg::from_usage("<input> 'input file.'")),
            SubCommand::with_name("is")
                .about("Determining whether is encrypted")
                .arg(Arg::from_usage("<input> 'input file.'"))
        ]);
    let mut help_buf = Vec::new();
    app.write_help(&mut help_buf).ok();
    let matches = app.get_matches();

    match matches.subcommand() {
        ("en", Some(sub)) => { write(
            sub.value_of("output").unwrap(),
            &encryptsave::pass_encrypt(
                sub.value_of("passwd").unwrap(),
                &read(sub.value_of("input").unwrap())
            ).unwrap()
        ) },
        ("de", Some(sub)) => { write(
            sub.value_of("output").unwrap(),
            &encryptsave::pass_decrypt(
                sub.value_of("passwd").unwrap(),
                &read(sub.value_of("input").unwrap())
            ).unwrap()
        ) },
        ("is", Some(sub)) => { println!(
            "{:?}",
            encryptsave::is_encrypted(&read(
                sub.value_of("input").unwrap()
            )))
        },
        _ => { stdout().write(&help_buf).ok(); }
    };
}
