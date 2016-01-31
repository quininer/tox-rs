use chrono::NaiveDateTime;
use super::{ ffi, error };


#[derive(Clone, Debug)]
pub struct Friend {
    pub core: *mut ffi::Tox,
    pub number: u32
}

impl Friend {
    pub fn new(core: *mut ffi::Tox, number: u32) -> Friend {
        Friend { core: core, number: number }
    }

    pub fn delete(&self) -> Result<(), error::DelFriendErr> {
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
