use super::{
    ffi, error, vars
};

pub struct Friend {
    core: *mut ffi::Tox,
    pub number: ::libc::uint32_t
}

impl Friend {
    pub fn new(core: *mut ffi::Tox, number: ::libc::uint32_t) -> Friend {
        Friend { core: core, number: number }
    }
}

pub trait Status {
    fn name(&self) -> Result<Vec<u8>, error::QueryFriendErr>;
    fn publickey(&self) -> Result<Vec<u8>, error::GetFriendPKErr>;
    fn status(&self) -> Result<vars::UserStatus, error::QueryFriendErr>;
    fn status_message(&self) -> Result<Vec<u8>, error::QueryFriendErr>;
    fn connection_status(&self) -> Result<vars::Connection, error::QueryFriendErr>;
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

    fn publickey(&self) -> Result<Vec<u8>, error::GetFriendPKErr> {
        out!( out
            out <- vec_with!(vars::TOX_PUBLIC_KEY_SIZE),
            err,
            ffi::tox_friend_get_public_key(
                self.core,
                self.number,
                out.as_mut_ptr(),
                &mut err
            )
        )
    }

    fn status(&self) -> Result<vars::UserStatus, error::QueryFriendErr> {
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

    fn connection_status(&self) -> Result<vars::Connection, error::QueryFriendErr> {
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