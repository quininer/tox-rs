extern crate tox;

use std::thread::sleep;
use tox::core::{ ToxOptions, Network, Status, Chat };
use tox::core::events::{ Listen, Event };

fn main() {
    let mut im = ToxOptions::new().generate().unwrap();
    im.set_name("bot").ok();
    println!("name: {}", String::from_utf8_lossy(&im.name().unwrap()));
    im.bootstrap("127.0.0.1", 33445, "269E0A8D082560545170ED8CF16D902615265B04F0E8AD82C7665DDFC3FF5A6C".parse().unwrap()).ok();

    let toxiter = im.iterate();
    loop {
        match toxiter.try_recv() {
            Ok(Event::SelfConnection(status)) => {
                println!("Self<{:?}> Connection: {:?}", im, status);
                im.request_friend(
                    "269E0A8D082560545170ED8CF16D902615265B04F0E8AD82C7665DDFC3FF5A6C14D2084C9529".parse().unwrap(),
                    "hi~"
                ).unwrap();
            },
            Ok(Event::FriendName(friend, name)) => {
                friend.say(format!("Hello {}", String::from_utf8_lossy(&name))).ok();
            },
            Err(_) => (),
            e @ _ => println!("Event: {:?}", e)
        }
        sleep(im.interval());
    }
}
