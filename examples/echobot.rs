extern crate tox;
extern crate rustc_serialize;

use std::thread::sleep;
use rustc_serialize::hex::ToHex;
use tox::core::{
    ToxOptions, Event,
    Network, Status, Chat, Listen,
    FriendManage, GroupCreate
};
use tox::core::group::GroupManage;


fn main() {
    let mut im = ToxOptions::new().generate().unwrap();
    im.set_name("echobot").ok();
    println!(
        "{}: {}",
        String::from_utf8_lossy(&im.name().unwrap()),
        &im.address()
    );
    im.bootstrap("127.0.0.1", 33445, "269E0A8D082560545170ED8CF16D902615265B04F0E8AD82C7665DDFC3FF5A6C".parse().unwrap()).ok();

    let toxiter = im.iterate();
    loop {
        match toxiter.try_recv() {
            Ok(Event::SelfConnection(status)) => {
                println!("Self<{:?}> Connection: {:?}", im, status);
            },
            Ok(Event::RequestFriend(pk, message)) => {
                println!(
                    "{}: {}",
                    pk.as_ref().to_hex(),
                    String::from_utf8_lossy(&message)
                );
                im.add_friend(pk).ok();
            },
            Ok(Event::FriendMessage(friend, message_type, message)) => {
                friend.send(
                    message_type,
                    message
                ).ok();
            },
            Ok(Event::GroupInvite(friend, _, token)) => {
                im.join(&friend, &token);
            },
            Ok(Event::GroupMessage(group, peer, message_type, message)) => {
                if !peer.is_ours() {
                    group.send(message_type, message).ok();
                    println!(
                        "peers {:?}",
                        group.peers().iter()
                            .map(|p| p.name().unwrap())
                            .map(|n| String::from_utf8(n).ok())
                            .collect::<Vec<Option<String>>>()
                    );
                }
            },
            Err(_) => (),
            e @ _ => println!("Event: {:?}", e)
        }
        sleep(im.interval());
    }
}
