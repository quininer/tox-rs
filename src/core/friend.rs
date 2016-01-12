use chrono::NaiveDateTime;
use super::chat::{ MessageType, MessageID };
use super::{
    ffi, error, vars, status,
    Address, PublicKey, Status, Chat
};

pub struct Friend {
    core: *mut ffi::Tox,
    pub number: u32
}

impl Friend {
    pub fn new(core: *mut ffi::Tox, number: ::libc::uint32_t) -> Friend {
        Friend { core: core, number: number }
    }
    pub fn delete(&mut self) -> Result<(), error::DelFriendErr> {
        out!( bool
            err,
            ffi::tox_friend_delete(
                self.core,
                self.number,
                &mut err
            )
        )
    }
    pub fn last(&self) -> Result<NaiveDateTime, error::GetFriendLastErr> {
        out!((num <- ::libc::uint64_t)
            err,
            ffi::tox_friend_get_last_online(
                self.core,
                self.number,
                &mut err
            )
        ).map(|r| NaiveDateTime::from_timestamp(r as i64, 0))
    }
    pub fn is_typing(&self) -> Result<bool, error::QueryFriendErr> {
        out!( err
            err,
            ffi::tox_friend_get_typing(
                self.core,
                self.number,
                &mut err
            )
        )
    }
}

impl Status for Friend {
    fn name(&self) -> Result<Vec<u8>, error::QueryFriendErr> {
        let len = try!(out!( err
            err,
            ffi::tox_friend_get_name_size(
                self.core,
                self.number,
                &mut err
            )
        ));
        out!( out
            out <- vec_with!(len),
            err,
            ffi::tox_friend_get_name(
                self.core,
                self.number,
                out.as_mut_ptr(),
                &mut err
            )
        )
    }

    fn address(&self) -> Result<Address, ()> {
        unimplemented!()
    }

    fn publickey(&self) -> Result<PublicKey, error::GetFriendPKErr> {
        out!( out
            out <- vec_with!(vars::TOX_PUBLIC_KEY_SIZE),
            err,
            ffi::tox_friend_get_public_key(
                self.core,
                self.number,
                out.as_mut_ptr(),
                &mut err
            )
        ).map(|r| PublicKey::new(&r))
    }

    fn nospam(&self) -> Result<u32, ()> {
        unimplemented!()
    }

    fn status(&self) -> Result<status::UserStatus, error::QueryFriendErr> {
        out!( err
            err,
            ffi::tox_friend_get_status(
                self.core,
                self.number,
                &mut err
            )
        )
    }

    fn status_message(&self) -> Result<Vec<u8>, error::QueryFriendErr> {
        let len = try!(out!( err
            err,
            ffi::tox_friend_get_status_message_size(
                self.core,
                self.number,
                &mut err
            )
        ));
        out!( out
            out <- vec_with!(len),
            err,
            ffi::tox_friend_get_status_message(
                self.core,
                self.number,
                out.as_mut_ptr(),
                &mut err
            )
        )
    }

    fn connection_status(&self) -> Result<status::Connection, error::QueryFriendErr> {
        out!( err
            err,
            ffi::tox_friend_get_connection_status(
                self.core,
                self.number,
                &mut err
            )
        )
    }
}

impl Chat for Friend {
    fn send<S: AsRef<[u8]>>(&mut self, ty: MessageType, message: S) -> Result<MessageID, error::SendMessageErr> {
        let message = message.as_ref();
        out!( err
            err,
            ffi::tox_friend_send_message(
                self.core,
                self.number,
                ty,
                message.as_ptr(),
                message.len(),
                &mut err
            )
        ).map_err(|err| error::SendMessageErr::from(err))
    }

    fn say<S: AsRef<[u8]>>(&mut self, message: S) -> Result<MessageID, error::SendMessageErr> {
        self.send(MessageType::NORMAL, message)
    }

    fn action<S: AsRef<[u8]>>(&mut self, message: S) -> Result<MessageID, error::SendMessageErr> {
        self.send(MessageType::ACTION, message)
    }
}
