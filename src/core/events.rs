use std::mem::transmute;
use std::time::Duration;
use std::sync::mpsc::{ channel, Sender, Receiver };
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
    fn _interval(&self) -> Duration;
    fn _iterate(&mut self);
    fn interval(&mut self) -> Duration {
        self._iterate();
        self._interval()
    }
    fn iterate(&mut self) -> Receiver<Event>;
}

impl Listen for Tox {
    fn _interval(&self) -> Duration {
        Duration::from_millis(unsafe { ffi::tox_iteration_interval(self.core) } as u64)
    }
    fn _iterate(&mut self) {
        unsafe { ffi::tox_iterate(self.core) }
    }

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
