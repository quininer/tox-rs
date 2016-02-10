use std::mem::transmute;
use super::{
    ffi, error, vars,
    Tox, Friend, Peer,
    Chat
};
use super::chat::{ MessageType, MessageID };
use_as! {
    TOX_CHAT_CHANGE as PeerChange
}


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GroupType {
    TEXT = 0,
    AV = 1
}


/// GroupChat (TEXT).
#[derive(Clone, Debug)]
pub struct Group {
    pub core: *mut ffi::Tox,
    pub number: i32
}

impl Group {
    pub fn from(core: *mut ffi::Tox, number: i32) -> Group {
        Group { core: core, number: number }
    }

    /// Leave GroupChat.
    pub fn leave(self) -> bool {
        unsafe { ffi::tox_del_groupchat(self.core, self.number) == 0 }
    }
    /// Invite friend to GroupChat.
    pub fn invite(&self, friend: &Friend) -> bool {
        unsafe { ffi::tox_invite_friend(
            self.core,
            friend.number as ::libc::int32_t,
            self.number
        ) == 0 }
    }
}


/// Create GroupChat.
pub trait GroupCreate {
    /// Create GroupChat.
    fn create_group(&self) -> Result<Group, ()>;
    /// Join GroupChat.
    fn join(&self, friend: &Friend, data: &[u8]) -> Result<Group, ()>;
}

impl GroupCreate for Tox {
    fn create_group(&self) -> Result<Group, ()> {
        match unsafe { ffi::tox_add_groupchat(self.core) } {
            -1 => Err(()),
            num @ _ => Ok(Group::from(self.core, num))
        }
    }
    fn join(&self, friend: &Friend, data: &[u8]) -> Result<Group, ()> {
        match unsafe { ffi::tox_join_groupchat(
            self.core,
            friend.number as ::libc::int32_t,
            data.as_ptr(),
            data.len() as ::libc::uint16_t
        ) } {
            -1 => Err(()),
            num @ _ => Ok(Group::from(self.core, num))
        }
    }
}


impl Chat for Group {
    /// old GroupChat API, MessageID only 0.
    fn send<S: AsRef<[u8]>>(&self, ty: MessageType, message: S) -> Result<MessageID, error::SendMessageErr> {
        match ty {
            MessageType::NORMAL => self.say(message),
            MessageType::ACTION => self.action(message)
        }
    }
    fn say<S: AsRef<[u8]>>(&self, message: S) -> Result<MessageID, error::SendMessageErr> {
        let message = message.as_ref();
        match unsafe { ffi::tox_group_message_send(
            self.core,
            self.number,
            message.as_ptr(),
            message.len() as ::libc::uint16_t
        ) } {
            0 => Ok(0),
            _ => Err(error::SendMessageErr::Group)
        }
    }
    fn action<S: AsRef<[u8]>>(&self, message: S) -> Result<MessageID, error::SendMessageErr> {
        let message = message.as_ref();
        match unsafe { ffi::tox_group_action_send(
            self.core,
            self.number,
            message.as_ptr(),
            message.len() as ::libc::uint16_t
        ) } {
            0 => Ok(0),
            _ => Err(error::SendMessageErr::Group)
        }
    }
}


/// Manage Groupchat.
pub trait GroupManage {
    /// Get Group Title.
    fn title(&self) -> Result<Vec<u8>, ()>;
    /// Set Group Title.
    fn set_title(&self, title: &[u8]) -> bool;
    /// Get Peer list.
    fn peers(&self) -> Result<Vec<Peer>, ()>;
    // fn peers_name(&self) -> Vec<Vec<u8>>;
    // fn peers_pk(&self) -> Vec<PublicKey>;
    /// Get Group type.
    fn get_type(&self) -> Result<GroupType, ()>;
}

impl GroupManage for Group {
    fn title(&self) -> Result<Vec<u8>, ()> {
        let mut out = unsafe { vec_with!(vars::TOX_MAX_NAME_LENGTH) };
        match unsafe { ffi::tox_group_get_title(
            self.core,
            self.number,
            out.as_mut_ptr(),
            out.len() as ::libc::uint32_t
        ) } {
            -1 => Err(()),
            _ => Ok(out)
        }
    }

    fn set_title(&self, title: &[u8]) -> bool {
        unsafe { ffi::tox_group_set_title(
            self.core,
            self.number,
            title.as_ptr(),
            title.len() as ::libc::uint8_t
        ) == 0 }
    }

    fn peers(&self) -> Result<Vec<Peer>, ()> {
        match unsafe { ffi::tox_group_number_peers(
            self.core,
            self.number
        ) } {
            -1 => Err(()),
            count @ _ => Ok(
                (0..count)
                    .map(|pn| Peer::from(self, pn))
                    .collect()
            )
        }
    }

    fn get_type(&self) -> Result<GroupType, ()> {
        unsafe { match ffi::tox_group_get_type(
            self.core,
            self.number
        ) {
            -1 => Err(()),
            ty @ _ => Ok(transmute(ty))
        } }
    }
}
