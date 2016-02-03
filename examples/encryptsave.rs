extern crate clap;
extern crate tox;

use std::io::stdout;
use std::fs::File;
use std::path::Path;
use std::io::{ Read, Write };
use clap::{ App, Arg, SubCommand };
use tox::encryptsave::{ ToxPassKey, is_encrypted };


fn read<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut data = Vec::new();
    File::open(path).unwrap().read_to_end(&mut data).unwrap();
    data
}

fn write<P: AsRef<Path>>(path: P, data: &[u8]) {
    File::create(path).unwrap().write(data).unwrap();
}

fn main() {
    let app = App::new("EncryptSave")
        .about("Tox-rs examples - encryptsave.")
        .args(&[
            Arg::from_usage("<input> 'input file.'"),
            Arg::from_usage("-p, --passwd [passphrase] 'passphrase.'").global(true),
            Arg::from_usage("-o, --output [file] 'out file.'").global(true)
        ])
        .subcommands(vec![
            SubCommand::with_name("en").about("Encryption"),
            SubCommand::with_name("de").about("Decryption"),
            SubCommand::with_name("is").about("Determining whether is encrypted")
        ]);
    let mut help_buf = Vec::new();
    app.write_help(&mut help_buf).ok();
    let matches = app.get_matches();

    match matches.subcommand() {
        ("en", Some(sub)) | ("de", Some(sub)) => {
            let passkey = ToxPassKey::new(
                sub.value_of("passwd").unwrap().as_bytes()
            ).unwrap();
            let input = read(matches.value_of("input").unwrap());
            let output = match matches.subcommand_name() {
                Some("en") => passkey.encrypt(&input).unwrap(),
                Some("de") => passkey.decrypt(&input).unwrap(),
                _ => unreachable!()
            };
            match sub.value_of("output") {
                Some(path) => write(path, &output),
                None => {
                    let mut stdout = std::io::stdout();
                    stdout.write(&output).unwrap();
                    stdout.flush().unwrap();
                }
            };
        },
        ("is", Some(_)) => { println!(
            "{:?}",
            is_encrypted(&read(
                matches.value_of("input").unwrap()
            )))
        },
        _ => { stdout().write(&help_buf).ok(); }
    };
}
