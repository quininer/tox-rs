extern crate clap;
extern crate tox;
extern crate ttyaskpass;

use std::fs::File;
use std::path::Path;
use std::io::{ stdout, Read, Write };
use clap::{ App, Arg, SubCommand };
use ttyaskpass::askpass;
use tox::encryptsave::{ pass_encrypt, pass_decrypt, is_encrypted };


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
        .about("Tox on Rust examples - encryptsave.")
        .args(&[
            Arg::from_usage("<INPUT> 'input file.'"),
            Arg::from_usage("-p, --passwd [Passphrase] 'Passphrase.'").global(true),
            Arg::from_usage("-o, --output [File] 'Out file.'").global(true)
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
            let passphrase: Vec<u8> = sub.value_of("passwd")
                .map(|s| s.into())
                .unwrap_or_else(|| askpass(b"~").unsecure().into());
            let input = read(matches.value_of("INPUT").unwrap());

            let output = match matches.subcommand_name() {
                Some("en") => pass_encrypt(&passphrase, &input).unwrap(),
                Some("de") => pass_decrypt(&passphrase, &input).unwrap(),
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
                matches.value_of("INPUT").unwrap()
            )))
        },
        _ => { stdout().write(&help_buf).ok(); }
    };
}
