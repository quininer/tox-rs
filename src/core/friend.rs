use chrono::NaiveDateTime;
use super::{ ffi, Tox, Address, PublicKey, error };


#[derive(Clone, Debug)]
pub struct Friend {
    pub core: *mut ffi::Tox,
    pub number: u32
}

impl Friend {
    pub fn from(core: *mut ffi::Tox, number: u32) -> Friend {
        Friend { core: core, number: number }
    }

    /// Delete Friend.
    pub fn delete(self) -> Result<(), error::DelFriendErr> {
        out!( bool
            err,
            ffi::tox_friend_delete(
                self.core,
                self.number,
                &mut err
            )
        )
    }

    /// Last Online time.
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

    /// Is Typing?
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

    /// Send Typing.
    pub fn set_typing(&self, typing: bool) -> Result<(), error::TypingSetErr> {
        out!( bool
            err,
            ffi::tox_self_set_typing(
                self.core,
                self.number,
                typing,
                &mut err
            )
        )
    }
}

pub trait FriendManage {
    /// Request Friend by Address.
    fn request_friend<S: AsRef<[u8]>>(&self, address: Address, message: S) -> Result<Friend, error::AddFriendErr>;
    /// Add Friend by PublicKey.
    fn add_friend(&self, public_key: PublicKey) -> Result<Friend, error::AddFriendErr>;
    /// Get Friend number by PublicKey.
    fn get_friend(&self, public_key: PublicKey) -> Result<Friend, error::PKGetFriendErr>;
    /// Friend exists?
    fn exists_friend(&self, friend: Friend) -> bool;
    /// Friend List.
    fn list_friend(&self) -> Vec<Friend>;
}

impl FriendManage for Tox {
    fn request_friend<S: AsRef<[u8]>>(&self, address: Address, message: S) -> Result<Friend, error::AddFriendErr> {
        let message = message.as_ref();
        out!( num
            err,
            ffi::tox_friend_add(
                self.core,
                address.out().as_ptr(),
                message.as_ptr(),
                message.len(),
                &mut err
            )
        ).map(|r| Friend::from(self.core, r))
    }
    fn add_friend(&self, public_key: PublicKey) -> Result<Friend, error::AddFriendErr> {
        out!( num
            err,
            ffi::tox_friend_add_norequest(
                self.core,
                public_key.as_ref().as_ptr(),
                &mut err
            )
        ).map(|r| Friend::from(self.core, r))
    }
    fn get_friend(&self, public_key: PublicKey) -> Result<Friend, error::PKGetFriendErr> {
        out!( num
            err,
            ffi::tox_friend_by_public_key(
                self.core,
                public_key.as_ref().as_ptr(),
                &mut err
            )
        ).map(|r| Friend::from(self.core, r))
    }
    fn exists_friend(&self, friend: Friend) -> bool {
        unsafe { ffi::tox_friend_exists(self.core, friend.number) }
    }
    fn list_friend(&self) -> Vec<Friend> {
        unsafe {
            let mut out = vec_with!(ffi::tox_self_get_friend_list_size(self.core));
            ffi::tox_self_get_friend_list(self.core, out.as_mut_ptr());
            out.iter()
                .map(|&r| Friend::from(self.core, r))
                .collect()
        }
    }
}
