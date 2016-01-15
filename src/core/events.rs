use std::time::Duration;
use std::mem::transmute;
use coio::sync::mpsc::{ channel, Sender, Receiver };
use libc::c_void;
use super::{
    ffi, vars,
    Tox, Friend,
    Network
};


#[derive(Clone, Debug)]
pub enum Event {
    SelfConnection(vars::Connection),
    FriendName(Friend, Vec<u8>)
}

pub trait Listen: Network {
    fn iterate(&mut self) -> Receiver<Event>;
    fn interval(&mut self) -> Duration {
        self._iterate();
        self._interval()
    }
}

impl Listen for Tox {
    fn iterate(&mut self) -> Receiver<Event> {
        let (sender, receiver) = channel::<Event>();

        unsafe {
            let tx: *mut c_void = transmute(Box::new(sender));

            ffi::tox_callback_self_connection_status(self.core, on_self_connection_status, tx);
        };

        receiver
    }
}

extern "C" fn on_self_connection_status(_: *mut ffi::Tox, connection_status: vars::Connection, tx: *mut c_void) {
    unsafe {
        let sender: Box<Sender<Event>> = transmute(tx);
        sender.send(Event::SelfConnection(connection_status)).ok();
    }
}
