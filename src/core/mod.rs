mod ffi;
pub mod error;
pub mod vars;
pub mod options;
pub mod status;
pub mod chat;
mod friend;
mod network;
mod address;
mod custom;

pub use core::options::ToxOptions;
pub use core::status::Status;
pub use core::network::Network;
pub use core::friend::Friend;
pub use core::address::{ PublicKey, Address };
pub use core::chat::Chat;
pub use core::custom::Packet;


#[derive(Clone, Debug)]
pub struct Tox {
    core: *mut ffi::Tox
}

impl Tox {
    pub fn new(opts: ToxOptions) -> Result<Tox, error::NewErr> {
        Ok(Tox { core: try!(out!(err err, ffi::tox_new(&opts.opts, &mut err))) })
    }

    pub fn save(&self) -> Vec<u8> {
        out!( get
            out <- vec_with!(ffi::tox_get_savedata_size(self.core)),
            ffi::tox_get_savedata(self.core, out.as_mut_ptr())
        )
    }

    pub fn secretkey(&self) -> Vec<u8> {
        out!( get
            out <- vec_with!(vars::TOX_SECRET_KEY_SIZE),
            ffi::tox_self_get_secret_key(self.core, out.as_mut_ptr())
        )
    }

    pub fn set_name<S: AsRef<[u8]>>(&mut self, name: S) -> Result<(), error::InfoSetErr> {
        let name = name.as_ref();
        out!( bool
            err,
            ffi::tox_self_set_name(
                self.core,
                name.as_ptr(),
                name.len(),
                &mut err
            )
        )
    }
    pub fn set_nospam(&mut self, nospam: u32) {
        unsafe { ffi::tox_self_set_nospam(self.core, nospam) }
    }
    pub fn set_status(&mut self, status: status::UserStatus) {
        unsafe { ffi::tox_self_set_status(self.core, status) }
    }
    pub fn set_status_message<S: AsRef<[u8]>>(&mut self, message: S) -> Result<(), error::InfoSetErr> {
        let message = message.as_ref();
        out!( bool
            err,
            ffi::tox_self_set_status_message(
                self.core,
                message.as_ptr(),
                message.len(),
                &mut err
            )
        )
    }
    pub fn set_typing(&mut self, friend: Friend, typing: bool) -> Result<(), error::TypingSetErr> {
        out!( bool
            err,
            ffi::tox_self_set_typing(
                self.core,
                friend.number,
                typing,
                &mut err
            )
        )
    }

    pub fn request_friend<S: AsRef<[u8]>>(&mut self, address: Address, message: S) -> Result<Friend, error::AddFriendErr> {
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
        ).map(|r| Friend::new(self.core, r))
    }
    pub fn add_friend(&mut self, public_key: PublicKey) -> Result<Friend, error::AddFriendErr> {
        out!( num
            err,
            ffi::tox_friend_add_norequest(
                self.core,
                public_key.as_ref().as_ptr(),
                &mut err
            )
        ).map(|r| Friend::new(self.core, r))
    }
    pub fn get_friend(&self, public_key: PublicKey) -> Result<Friend, error::PKGetFriendErr> {
        out!( num
            err,
            ffi::tox_friend_by_public_key(
                self.core,
                public_key.as_ref().as_ptr(),
                &mut err
            )
        ).map(|r| Friend::new(self.core, r))
    }
    pub fn exists_friend(&self, friend: Friend) -> bool {
        unsafe { ffi::tox_friend_exists(self.core, friend.number) }
    }
    pub fn list_friend(&self) -> Vec<Friend> {
        unsafe {
            let mut out = vec_with!(ffi::tox_self_get_friend_list_size(self.core));
            ffi::tox_self_get_friend_list(self.core, out.as_mut_ptr());
            out.iter()
                .map(|&r| Friend::new(self.core, r))
                .collect()
        }
    }
}

impl Drop for Tox {
    fn drop(&mut self) {
        unsafe { ffi::tox_kill(self.core); }
    }
}
