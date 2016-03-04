extern crate tox;

use std::fs::File;
use std::path::Path;
use std::io::{ Write, Read };
use std::thread::sleep;
use tox::core::{
    ToxOptions, Event,
    Network, Status, Chat, Listen,
    FriendManage
};
use tox::core::file::{ FileKind, FileControl, FileOperate, FileManage };
use tox::av::{ ToxAv, AvEvent, AvSend, Call };
use tox::av::toav::{ ToAv, ToTox };

#[cfg(feature = "groupchat")]
use tox::core::group::{ GroupManage, GroupCreate, GroupType };

#[cfg(feature = "groupchat")]
use tox::av::AvGroupCreate;


macro_rules! try_read {
    ( $path:expr ) => {
        if $path.is_file() {
            let mut data = Vec::new();
            File::open($path).unwrap()
                .read_to_end(&mut data).unwrap();
            ToxOptions::new().from(&data).generate()
        } else {
            ToxOptions::new().generate()
        }.unwrap();
    }
}

fn main() {
    let path = Path::new("echobot.tox");
    let mut im = try_read!(path);
    let mut imav = ToxAv::new(&im).unwrap();

    im.set_name("echobot").ok();
    println!("{}", &im.address());
    im.bootstrap("127.0.0.1:33445", "269E0A8D082560545170ED8CF16D902615265B04F0E8AD82C7665DDFC3FF5A6C".parse().unwrap()).ok();
    let mut buffer: Vec<u8> = Vec::new();

    let toxiter = im.iterate();
    let aviter = imav.iterate();

    'main: loop {
        sleep(im.interval());
        match toxiter.try_recv() {
            Ok(Event::SelfConnection(status)) => {
                println!("Self Connection: {:?}", status);
            },
            Ok(Event::RequestFriend(pk, _)) => {
                if im.add_friend(pk).is_ok() {
                    File::create(path).unwrap()
                        .write(&im.save()).unwrap();
                }
            },
            Ok(Event::FriendMessage(friend, message_type, message)) => {
                match message.as_slice() {
                    b"hi" => {
                        friend.say(format!(
                            "hi, {}.",
                            String::from_utf8_lossy(&friend.name().unwrap())
                        )).ok();
                    },
                    b"save" => {
                        File::create("recvfile").unwrap()
                            .write(&buffer).unwrap();
                    },
                    b"clean" => {
                        buffer = Vec::new();
                    },
                    b"avatar" => {
                        friend.transmission(
                            FileKind::AVATAR,
                            "avatar.png",
                            buffer.len() as u64,
                            None
                        ).unwrap();
                    },
                    b"call me" => {
                        friend.to_av(&imav).call(48, 0).unwrap();
                    },
                    b"exit" => break 'main,
                    msg @ _ => { friend.send(message_type, msg).ok(); }
                };
            },
            Ok(Event::FriendFileChunkRequest(_, file, pos, size)) => {
                file.send(pos, &buffer[pos as usize .. pos as usize+size]).ok();
            },

            Ok(Event::FriendFileRecv(friend, kind, file, size, name)) => {
                friend.say(format!(
                    "{} - {:?} - {} - {:?}",
                    String::from_utf8_lossy(&name),
                    kind,
                    size,
                    file.get_id()
                )).ok();
                file.control(FileControl::RESUME).unwrap();
            },
            Ok(Event::FriendFileRecvChunk(_, _, _, data)) => {
                buffer = [buffer, data].concat();
            },

            #[cfg(feature = "groupchat")]
            Ok(Event::GroupInvite(friend, ty, token)) => {
                match ty {
                    GroupType::TEXT => im.join(&friend, &token).ok(),
                    GroupType::AV => {
                        im.join_av(&friend, &token, Box::new(|g, _, _, _, _, _| {
                            g.say("meow~").ok();
                        })).ok()
                    }
                };
            },

            #[cfg(feature = "groupchat")]
            Ok(Event::GroupMessage(group, peer, message_type, message)) => {
                if !peer.is_ours() {
                    group.send(message_type, message).ok();
                };
                println!(
                    "Group {} Peers {:?}",
                    group.number,
                    group.peers().unwrap().iter()
                        .map(|p| p.name().unwrap())
                        .map(|n| String::from_utf8(n).ok())
                        .collect::<Vec<Option<String>>>()
                );
            },

            Err(_) => (),
            e @ _ => println!("Event: {:?}", e)
        };

        imav._iterate();
        match aviter.try_recv() {
            Ok(AvEvent::FriendCall(avfriend, a, v)) => {
                avfriend.to_tox(&im).say("Av~").unwrap();
                avfriend.answer(
                    if a { 48 } else { 0 },
                    if v { 500 } else { 0 }
                ).unwrap();
            },
            Ok(AvEvent::FriendAudioFrameReceive(avfriend, pcm, count, chan, rate)) => {
                // TODO save to file
                avfriend.send_audio(&pcm, count, chan, rate).ok();
            },
            Ok(AvEvent::FriendVideoFrameReceive(avfriend, w, h, y, u, v, ys, us, vs)) => {
                // TODO save to file
                // FIXME ugly
                let mut yy = Vec::new();
                for i in 0..h {
                    let mut yyy = y[(i as usize * ys as usize) .. ((i as usize * ys as usize) + w as usize)].into();
                    yy.append(&mut yyy);
                }

                let mut uu = Vec::new();
                let mut vv = Vec::new();
                for i in 0..(h as usize / 2) {
                    let mut uuu = u[(i as usize * us as usize) .. ((i as usize * us as usize) + w as usize / 2)].into();
                    let mut vvv = v[(i as usize * vs as usize) .. ((i as usize * vs as usize) + w as usize / 2)].into();
                    uu.append(&mut uuu);
                    vv.append(&mut vvv);
                }
                avfriend.send_video(w, h, &yy, &uu, &vv).ok();
            },
            Err(_) => (),
            e @ _ => println!("AvEvent: {:?}", e)
        };
    }
}
