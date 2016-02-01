extern crate tox;

use std::fs::File;
use std::io::Write;
use std::thread::sleep;
use tox::core::{
    ToxOptions, Event,
    Network, Status, Chat, Listen,
    FriendManage
};
use tox::core::file::{ FileControl, FileOperate };

#[cfg(feature = "groupchat")]
use tox::core::group::{ GroupManage, GroupCreate };


fn main() {
    let mut im = ToxOptions::new().generate().unwrap();
    im.set_name("echobot").ok();
    println!("{}", &im.address());
    im.bootstrap("127.0.0.1", 33445, "269E0A8D082560545170ED8CF16D902615265B04F0E8AD82C7665DDFC3FF5A6C".parse().unwrap()).ok();
    let mut buffer: Vec<u8> = Vec::new();

    let toxiter = im.iterate();
    'main: loop {
        sleep(im.interval());
        match toxiter.try_recv() {
            Ok(Event::SelfConnection(status)) => {
                println!("Self Connection: {:?}", status);
            },
            Ok(Event::RequestFriend(pk, _)) => {
                im.add_friend(pk).ok();
            },
            Ok(Event::FriendMessage(friend, message_type, message)) => {
                match message.as_slice() {
                    b"save" => {
                        File::create("recvfile").unwrap()
                            .write(&buffer).unwrap();
                    },
                    b"exit" => break 'main,
                    msg @ _ => { friend.send(message_type, msg).ok(); }
                };
            },

            Ok(Event::FriendFileRecv(friend, kind, file, size, name)) => {
                friend.say(format!(
                    "{} - {:?} - {} - {:?}",
                    String::from_utf8_lossy(&name),
                    kind,
                    size,
                    file.get_id()
                )).ok();
                file.control(FileControl::RESUME).ok();
            },
            Ok(Event::FriendFileRecvChunk(_, _, _, data)) => {
                buffer = [buffer, data].concat();
            },

            #[cfg(feature = "groupchat")]
            Ok(Event::GroupInvite(friend, _, token)) => {
                im.join(&friend, &token);
            },

            #[cfg(feature = "groupchat")]
            Ok(Event::GroupMessage(group, peer, message_type, message)) => {
                if !peer.is_ours() {
                    group.send(message_type, message).ok();
                };
                println!(
                    "Group {} Peers {:?}",
                    group.number,
                    group.peers().iter()
                        .map(|p| p.name().unwrap())
                        .map(|n| String::from_utf8(n).ok())
                        .collect::<Vec<Option<String>>>()
                );
            },

            Err(_) => (),
            e @ _ => println!("Event: {:?}", e)
        }
    }
}
