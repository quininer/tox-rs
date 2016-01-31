use std::slice;
use std::mem::transmute;
use std::time::Duration;
use std::sync::mpsc::{ channel, Sender, Receiver };
use libc::c_void;
use super::{
    ffi, status,
    Tox, Friend,
    Network
};


#[derive(Clone, Debug)]
pub enum Event {
    SelfConnection(status::Connection),
    FriendStatus(Friend, status::UserStatus),
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

            // macro_rule! callback {}
            ffi::tox_callback_self_connection_status(self.core, on_self_connection_status, tx);
            ffi::tox_callback_friend_status(self.core, on_friend_status, tx);
            ffi::tox_callback_friend_name(self.core, on_friend_name, tx);
        };

        receiver
    }
}

extern "C" fn on_self_connection_status(
    _: *mut ffi::Tox,
    connection_status: status::Connection,
    tx: *mut c_void
) {
    unsafe {
        println!("status");
        let sender: &Sender<Event> = transmute(tx);
        sender.send(Event::SelfConnection(connection_status)).ok();
    }
}
extern "C" fn on_friend_status(
    core: *mut ffi::Tox,
    friend_number: ::libc::uint32_t,
    status: status::UserStatus,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        sender.send(Event::FriendStatus(
            Friend::new(core, friend_number),
            status
        )).ok();
    }
}

extern "C" fn on_friend_name(
    core: *mut ffi::Tox,
    friend_number: ::libc::uint32_t,
    name: *const ::libc::uint8_t,
    length: ::libc::size_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        sender.send(Event::FriendName(
            Friend::new(core, friend_number),
            slice::from_raw_parts(name, length).to_vec()
        )).ok();
    }
}
