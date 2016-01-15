extern crate tox;
extern crate coio;

use tox::core::{ ToxOptions, Network, Status };
use tox::core::events::{ Listen, Event };
use coio::{ Scheduler, sleep };

fn main() {
    let mut im = ToxOptions::new().generate().unwrap();
    im.set_name("bot").ok();
    println!("name: {}", String::from_utf8_lossy(&im.name().unwrap()));
    im.bootstrap("127.0.0.1", 33445, "EDF5A5BE8DFFC1DDFAACC71A0C0FCEEDE7BED4F3FBF9C54D502BE66A297DC374".parse().unwrap()).ok();

    Scheduler::new().run(move || {
        let iter = im.iterate();
        loop{
            if let Ok(ev) = iter.try_recv() {
                match ev {
                    Event::SelfConnection(status) => {
                        println!("Self<{:?}> Connection: {:?}", im, status);
                    },
                    e @ _ => println!("Event: {:?}", e)
                }
            };
            sleep(im.interval());
        }
    }).ok();
}
