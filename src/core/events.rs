use std::slice;
use std::mem::transmute;
use std::time::Duration;
use std::sync::mpsc::{ channel, Sender, Receiver };
use libc::*;
use super::chat::{ MessageID, MessageType };
use super::file::{ FileKind, FileControl };

#[cfg(feature = "groupchat")]
use super::group::{ GroupType, PeerChange, Group };

#[cfg(feature = "groupchat")]
use super::peer::Peer;

use super::{
    ffi, status, vars,
    Tox, Friend, PublicKey, File,
    Network
};


#[derive(Clone, Debug)]
pub enum Event {
    /// Self Connection Status.
    SelfConnection(status::Connection),
    /// Friend Request, `[PublicKey, Message]`.
    RequestFriend(PublicKey, Vec<u8>),

    // Friend status
    /// Friend Name, `[Friend, Name]`.
    FriendName(Friend, Vec<u8>),
    /// Friend Status Message, `[Friend, Status Message]`.
    FriendStatusMessage(Friend, Vec<u8>),
    /// Friend Status.
    FriendStatus(Friend, status::UserStatus),
    /// Friend Connection Status.
    FriendConnection(Friend, status::Connection),

    // Friend basic Message
    /// Friend Typing, `[Friend, is Typing]`.
    FriendTyping(Friend, bool),
    /// Friend Receipt, (no read).
    FriendReadReceipt(Friend, MessageID),
    /// Friend Message, `[Friend, Message Type, Message]`.
    FriendMessage(Friend, MessageType, Vec<u8>),

    // Friend Custom Packet
    /// Friend Lossy Packet, `[Friend, Data]`.
    FriendLossyPacket(Friend, Vec<u8>),
    /// Friend Lossless Packet, `[Friend, Data]`.
    FriendLosslessPacket(Friend, Vec<u8>),

    // Firend File
    /// Friend Request File Seek. `[Friend, File, position, length]`.
    FriendFileSeek(Friend, File<Friend>, u64, usize),
    /// Friend File Recv. `[Friend, FileKind, File, size, data]`.
    FriendFileRecv(Friend, FileKind, File<Friend>, u64, Vec<u8>),
    /// Friend File Chunk Recv. `[Friend, File, position, data]`.
    FriendFileRecvChunk(Friend, File<Friend>, u64, Vec<u8>),
    /// Friend File Control.
    FriendFileControl(Friend, File<Friend>, FileControl),

    // Group (Old API)
    #[cfg(feature = "groupchat")]
    /// Group Invite, `[Friend, Group Type, Token]`.
    GroupInvite(Friend, GroupType, Vec<u8>),
    #[cfg(feature = "groupchat")]
    /// Group Message, `[Friend, Peer, Message Type, Message]`.
    GroupMessage(Group, Peer, MessageType, Vec<u8>),
    #[cfg(feature = "groupchat")]
    /// Group Title, `[Friend, Peer or None, Message]`.
    GroupTitle(Group, Option<Peer>, Vec<u8>),
    #[cfg(feature = "groupchat")]
    /// Group Peer Change.
    GroupPeerChange(Group, Peer, PeerChange)
}


/// Listen Events.
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

            callback!( (self.core, tx),
                self_connection_status,

                // friend
                friend_request,
                friend_name,
                friend_status_message,
                friend_status,
                friend_connection_status,
                friend_typing,
                friend_read_receipt,
                friend_message,
                friend_lossy_packet,
                friend_lossless_packet,

                // file
                file_recv_control,
                file_chunk_request,
                file_recv,
                file_recv_chunk
            );

            #[cfg(feature = "groupchat")]
            callback!( (self.core, tx),
                group_invite,
                group_message,
                group_action,
                group_title,
                group_namelist_change
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
            Friend::from(core, friend_number),
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
            Friend::from(core, friend_number),
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
            Friend::from(core, friend_number),
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
            Friend::from(core, friend_number),
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
            Friend::from(core, friend_number),
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
            Friend::from(core, friend_number),
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
            Friend::from(core, friend_number),
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
            Friend::from(core, friend_number),
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
            Friend::from(core, friend_number),
            slice::from_raw_parts(data, length).to_vec()
        )).ok();
    }
}

extern "C" fn on_file_recv_control(
    core: *mut ffi::Tox,
    friend_number: uint32_t,
    file_number: uint32_t,
    control: FileControl,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        let friend = Friend::from(core, friend_number);
        let file = File::from(friend.clone(), file_number);
        sender.send(Event::FriendFileControl(
            friend,
            file,
            control
        )).ok();
    }
}

extern "C" fn on_file_chunk_request(
    core: *mut ffi::Tox,
    friend_number: uint32_t,
    file_number: uint32_t,
    position: uint64_t,
    length: size_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        let friend = Friend::from(core, friend_number);
        let file = File::from(friend.clone(), file_number);
        sender.send(Event::FriendFileSeek(
            friend,
            file,
            position,
            length
        )).ok();
    }
}

extern "C" fn on_file_recv(
    core: *mut ffi::Tox,
    friend_number: uint32_t,
    file_number: uint32_t,
    kind: uint32_t,
    file_size: u64,
    filename: *const uint8_t,
    filename_len: size_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        let friend = Friend::from(core, friend_number);
        let file = File::from(friend.clone(), file_number);
        let kind = match kind {
            1 => FileKind::AVATAR,
            0 | _ => FileKind::DATA,
        };
        sender.send(Event::FriendFileRecv(
            friend,
            kind,
            file,
            file_size,
            slice::from_raw_parts(filename, filename_len).to_vec()
        )).ok();
    }
}

extern "C" fn on_file_recv_chunk(
    core: *mut ffi::Tox,
    friend_number: uint32_t,
    file_number: uint32_t,
    position: uint64_t,
    data: *const uint8_t,
    length: size_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        let friend = Friend::from(core, friend_number);
        let file = File::from(friend.clone(), file_number);
        sender.send(Event::FriendFileRecvChunk(
            friend,
            file,
            position,
            slice::from_raw_parts(data, length).to_vec()
        )).ok();
    }
}

#[cfg(feature = "groupchat")]
extern "C" fn on_group_invite(
    core: *mut ffi::Tox,
    friend_number: int32_t,
    group_type: uint8_t,
    data: *const uint8_t,
    length: uint16_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        sender.send(Event::GroupInvite(
            Friend::from(core, friend_number as u32),
            transmute(group_type as uint32_t),
            slice::from_raw_parts(data, length as usize).to_vec()
        )).ok();
    }
}

#[cfg(feature = "groupchat")]
extern "C" fn on_group_message(
    core: *mut ffi::Tox,
    group_number: c_int,
    peer_number: c_int,
    message: *const uint8_t,
    length: uint16_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        let group = Group::from(core, group_number);
        let peer = Peer::from(&group, peer_number);
        sender.send(Event::GroupMessage(
            group,
            peer,
            MessageType::NORMAL,
            slice::from_raw_parts(message, length as usize).to_vec()
        )).ok();
    }
}

#[cfg(feature = "groupchat")]
extern "C" fn on_group_action(
    core: *mut ffi::Tox,
    group_number: c_int,
    peer_number: c_int,
    action: *const uint8_t,
    length: uint16_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        let group = Group::from(core, group_number);
        let peer = Peer::from(&group, peer_number);
        sender.send(Event::GroupMessage(
            group,
            peer,
            MessageType::ACTION,
            slice::from_raw_parts(action, length as usize).to_vec()
        )).ok();
    }
}

#[cfg(feature = "groupchat")]
extern "C" fn on_group_title(
    core: *mut ffi::Tox,
    group_number: c_int,
    peer_number: c_int,
    title: *const uint8_t,
    length: uint8_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        let group = Group::from(core, group_number);
        let peer_or = match peer_number {
            -1 => None,
            num @ _ => Some(Peer::from(&group, num))
        };
        sender.send(Event::GroupTitle(
            group,
            peer_or,
            slice::from_raw_parts(title, length as usize).to_vec()
        )).ok();
    }
}

#[cfg(feature = "groupchat")]
extern "C" fn on_group_namelist_change(
    core: *mut ffi::Tox,
    group_number: c_int,
    peer_number: c_int,
    change: uint8_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<Event> = transmute(tx);
        let group = Group::from(core, group_number);
        let peer = Peer::from(&group, peer_number);
        sender.send(Event::GroupPeerChange(
            group,
            peer,
            transmute(change as uint32_t)
        )).ok();
    }
}
