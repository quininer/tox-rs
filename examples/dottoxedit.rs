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
use tox::core::{ Tox, ToxOptions, Status, FriendManage };
use tox::encryptsave::{ is_encrypted, ToxPassKey };
use tox::address::PublicKey;


fn read<P: AsRef<Path>>(path: P, passphrase: Option<&str>) -> (Tox, Option<SecStr>) {
    let mut data = Vec::new();
    File::open(path).unwrap().read_to_end(&mut data).unwrap();
    if is_encrypted(&data) {
        let passphrase = match passphrase {
            Some(pass) => pass.into(),
            None => askpass(b"~")
        };
        let data = ToxPassKey::from(passphrase.clone(), &data).unwrap()
            .decrypt(&data).unwrap();
        (ToxOptions::default().from(&data).generate().unwrap(), Some(passphrase))
    } else {
        (ToxOptions::default().from(&data).generate().unwrap(), None)
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
    let app = App::new("Edit .TOX")
        .about("Tox on Rust examples - dottoxedit.")
        .args(&[
            Arg::from_usage("<PROFILE> '.tox file.'"),
            Arg::from_usage("--create 'Create new Profile.'").global(true),
            Arg::from_usage("-p, --passwd [String] 'Profile passphrase'").global(true)
        ])
        .subcommands(vec![
            SubCommand::with_name("self").about("Show self information."),
            SubCommand::with_name("list").about("Show friend list"),
            SubCommand::with_name("set").about("Edit self information.")
                .args(&[
                    Arg::from_usage("--name [String] 'set name.'"),
                    Arg::from_usage("--message [String] 'set status message.'"),
                    Arg::from_usage("--nospam [u32] 'set nospam code.'"),
                    Arg::from_usage("--passphrase [String] 'set passphrase.'"),
                ]),
            SubCommand::with_name("add").about("Add Friend.")
                .arg(Arg::from_usage("<PublicKey>... 'Friend Public Key.'")),
            SubCommand::with_name("del").about("Delete Friend.")
                .arg(Arg::from_usage("<PublicKey>... 'Friend Public Key.'")),
            SubCommand::with_name("merge").about("Merge other Profile.")
                .arg(Arg::from_usage("<OTHER> 'other Profile.'")),
            SubCommand::with_name("miss").about("Diff Firends with other Profile.")
                .arg(Arg::from_usage("<OTHER> 'other Profile.'"))
        ]);
    let mut help_buf = Vec::new();
    app.write_help(&mut help_buf).ok();
    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some("self") | Some("list") | Some("set") | Some("add") | Some("del") | Some("merge") | Some("miss") => {
            let path = matches.value_of("PROFILE").expect("missing profile.");
            let (tox, mut passphrase) = read(path, matches.value_of("passwd"));

            match matches.subcommand() {
                ("self", Some(_)) => {
                    println!(
                        "{} [{}]
                            {}",
                        String::from_utf8_lossy(&tox.name().unwrap()),
                        tox.address(),
                        String::from_utf8_lossy(&tox.status_message().unwrap())
                    );
                },
                ("list", Some(_)) => {
                    for friend in tox.list_friend() {
                        println!(
                            "{} [{}]
                            {}  {}",
                            String::from_utf8_lossy(&friend.name().unwrap()),
                            friend.publickey().unwrap(),
                            friend.last().unwrap(),
                            String::from_utf8_lossy(&friend.status_message().unwrap())
                        );
                    }
                },
                ("set", Some(sub)) => {
                    if let Some(name) = sub.value_of("name") {
                        tox.set_name(name).unwrap();
                    }
                    if let Some(status) = sub.value_of("message") {
                        tox.set_status_message(status).unwrap();
                    }
                    if let Some(nospam) = sub.value_of("nospam") {
                        tox.set_nospam(nospam.parse().unwrap());
                    }
                    if let Some(pass) = sub.value_of("passphrase") {
                        passphrase = Some(if pass.len() == 0 { askpass(b"~") } else { pass.into() });
                    }
                },
                ("add", Some(sub)) => {
                    for pk in sub.values_of("PublicKey").expect("missing PublicKey.") {
                        tox.add_friend(pk.parse().unwrap()).unwrap();
                    }
                },
                ("del", Some(sub)) => {
                    for pk in sub.values_of("PublicKey").expect("missing PublicKey") {
                        tox.get_friend(pk.parse().unwrap()).unwrap()
                            .delete().unwrap();
                    }
                },
                ("merge", Some(sub)) | ("miss", Some(sub)) => {
                    let (otox, _) = read(
                        sub.value_of("OTHER").expect("missing profile."),
                        None
                    );
                    let mpks = tox.list_friend().iter()
                        .map(|f| f.publickey().unwrap())
                        .collect::<Vec<PublicKey>>();
                    let opks = otox.list_friend().iter()
                        .map(|f| f.publickey().unwrap())
                        .collect::<Vec<PublicKey>>();
                    for pk in opks {
                        if !{
                            let mut result = false;
                            for e in &mpks {
                                result = &pk == e || result;
                            }
                            result
                        } {
                            match matches.subcommand_name() {
                                Some("merge") => { tox.add_friend(pk).unwrap(); },
                                Some("miss") => println!("{}", pk),
                                _ => unreachable!()
                            }
                        }
                    }
                },
                _ => unreachable!()
            };

            write(path, &tox.save(), passphrase);
        },
        _ => { stdout().write(&help_buf).ok(); }
    }
}
