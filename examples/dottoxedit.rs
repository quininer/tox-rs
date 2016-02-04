extern crate clap;
extern crate tox;
extern crate ttyaskpass;
extern crate secstr;

use std::fs::File;
use std::path::Path;
use std::io::{ stdout, Read, Write };
use clap::{ App, Arg, SubCommand };
use secstr::SecStr;
use ttyaskpass::askpass;
use tox::encryptsave::{ is_encrypted, ToxPassKey };


fn read<P: AsRef<Path>>(path: P, passphrase: Option<SecStr>) -> Vec<u8> {
    let mut data = Vec::new();
    File::open(path).unwrap().read_to_end(&mut data).unwrap();
    match passphrase {
        Some(pass) => ToxPassKey::from(pass, &data).unwrap()
            .decrypt(&data).unwrap(),
        None => data
    }
}

fn write<P: AsRef<Path>>(path: P, data: &[u8], passphrase: Option<SecStr>) {
    File::create(path).unwrap().write(&match passphrase {
        Some(pass) => ToxPassKey::new(pass).unwrap()
            .encrypt(data).unwrap(),
        None => data.into()
    }).unwrap();
}

fn main() {
    let app = App::new(" Edit .TOX")
        .about("Tox on Rust examples - dottoxedit.")
        .args(&[
            Arg::from_usage("<PROFILE> '.tox file.'"),
            Arg::from_usage("--create 'Create new Profile.'").global(true),
            Arg::from_usage("-p, --passwd [Passphrase] 'Profile passphrase'").global(true)
        ])
        .subcommands(vec![
            SubCommand::with_name("self").about("Edit self information.")
                .args(&[
                    Arg::from_usage("--name [String] 'show or set name.'"),
                    Arg::from_usage("--status [Status] 'show or set status (online/away/busy).'"),
                    Arg::from_usage("--message [String] 'show or set status message.'"),
                    Arg::from_usage("--nospam [Hex] 'show or set nospam code.'"),
                    Arg::from_usage("--passphrase [String] 'set passphrase.'"),
                    Arg::from_usage("--id 'show address.'")
                ]),
            SubCommand::with_name("add").about("Add Friend.")
                .arg(Arg::from_usage("<Publickey> 'Friend Public Key.'")),
            SubCommand::with_name("del").about("Delete Friend.")
                .args(&[
                      Arg::from_usage("[Publickey] 'Friend Public Key.'"),
                      Arg::from_usage("--name [String] 'delete friend, with name.'")
                ]),
            SubCommand::with_name("list").about("Friend list."),
            SubCommand::with_name("merge").about("Merge other Profile.")
                .arg(Arg::from_usage("<OTHER>... 'other Profile.'"))
        ]);
    let mut help_buf = Vec::new();
    app.write_help(&mut help_buf).ok();
    let matches = app.get_matches();

    match matches.subcommand() {
        _ => { stdout().write(&help_buf).ok(); }
    }
}
