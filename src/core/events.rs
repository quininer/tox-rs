use std::slice;
use std::mem::transmute;
use std::time::Duration;
use std::sync::mpsc::{ channel, Sender, Receiver };
use libc::*;
use super::chat::{ MessageID, MessageType };
use super::{
    ffi, status, vars,
    Tox, Friend, PublicKey,
    Network
};


#[derive(Clone, Debug)]
pub enum Event {
    SelfConnection(status::Connection),
    RequestFriend(PublicKey, Vec<u8>),

    // Friend status
    FriendName(Friend, Vec<u8>),
    FriendStatusMessage(Friend, Vec<u8>),
    FriendStatus(Friend, status::UserStatus),
    FriendConnection(Friend, status::Connection),

    // Friend basic Message
    FriendTyping(Friend, bool),
    FriendReadReceipt(Friend, MessageID),
    FriendMessage(Friend, MessageType, Vec<u8>),

    // Friend Custom Packet
    FriendLossyPacket(Friend, Vec<u8>),
    FriendLosslessPacket(Friend, Vec<u8>),

    // TODO
    // Firend File
    // FriendRecv(Friend, File),
    // FriendRecvChunk(Friend, FileChunk),

    // TODO
    // Old API
    // Group
    // GroupInvite(Friend, Group),
    // GroupMessage(Group, Peer, Vec<u8>),
    // GroupTitle(Group, Peer, Vec<u8>),
    // GroupPeerChange(Group, Peer, PeerAction)
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

            macro_rules! callback {
                ( $( $event:ident ),* ) => {{
                    use super::ffi::*;
                    $(
                        concat_idents!(tox_callback_, $event)(self.core, concat_idents!(on_, $event), tx);
                    )*
                }}
            }

            callback!(
                self_connection_status,
                friend_request,
                friend_name,
                friend_status_message,
                friend_status,
                friend_connection_status,
                friend_typing,
                friend_read_receipt,
                friend_message,
                friend_lossy_packet,
                friend_lossless_packet
            );
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
        let sender: &Sender<Event> = transmute(tx);
        sender.send(Event::SelfConnection(connection_status)).ok();
    }
}

extern "C" fn on_friend_request(
    _: *mut ffi::Tox,
    public_key: *const uint8_t,
    message: *const uint8_t,
    length: size_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        sender.send(Event::RequestFriend(
            PublicKey::from(slice::from_raw_parts(public_key, vars::TOX_PUBLIC_KEY_SIZE).to_vec()),
            slice::from_raw_parts(message, length).to_vec()
        )).ok();
    }
}

extern "C" fn on_friend_name(
    core: *mut ffi::Tox,
    friend_number: uint32_t,
    name: *const uint8_t,
    length: size_t,
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

extern "C" fn on_friend_status_message(
    core: *mut ffi::Tox,
    friend_number: uint32_t,
    message: *const uint8_t,
    length: size_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        sender.send(Event::FriendStatusMessage(
            Friend::new(core, friend_number),
            slice::from_raw_parts(message, length).to_vec()
        )).ok();
    }
}

extern "C" fn on_friend_status(
    core: *mut ffi::Tox,
    friend_number: uint32_t,
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

extern "C" fn on_friend_connection_status(
    core: *mut ffi::Tox,
    friend_number: uint32_t,
    connection_status: status::Connection,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        sender.send(Event::FriendConnection(
            Friend::new(core, friend_number),
            connection_status
        )).ok();
    }
}

extern "C" fn on_friend_typing(
    core: *mut ffi::Tox,
    friend_number: uint32_t,
    typing: bool,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        sender.send(Event::FriendTyping(
            Friend::new(core, friend_number),
            typing
        )).ok();
    }
}

extern "C" fn on_friend_read_receipt(
    core: *mut ffi::Tox,
    friend_number: uint32_t,
    message_id: uint32_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        sender.send(Event::FriendReadReceipt(
            Friend::new(core, friend_number),
            message_id
        )).ok();
    }
}

extern "C" fn on_friend_message(
    core: *mut ffi::Tox,
    friend_number: uint32_t,
    message_type: MessageType,
    message: *const uint8_t,
    length: size_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        sender.send(Event::FriendMessage(
            Friend::new(core, friend_number),
            message_type,
            slice::from_raw_parts(message, length).to_vec()
        )).ok();
    }
}

extern "C" fn on_friend_lossy_packet(
    core: *mut ffi::Tox,
    friend_number: uint32_t,
    data: *const uint8_t,
    length: size_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        sender.send(Event::FriendLossyPacket(
            Friend::new(core, friend_number),
            slice::from_raw_parts(data, length).to_vec()
        )).ok();
    }
}

extern "C" fn on_friend_lossless_packet(
    core: *mut ffi::Tox,
    friend_number: uint32_t,
    data: *const uint8_t,
    length: size_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        sender.send(Event::FriendLosslessPacket(
            Friend::new(core, friend_number),
            slice::from_raw_parts(data, length).to_vec()
        )).ok();
    }
}
