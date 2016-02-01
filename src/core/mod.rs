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
mod events;

#[cfg(feature = "groupchat")]
pub mod group;

#[cfg(feature = "groupchat")]
pub mod peer;

pub use self::options::ToxOptions;
pub use self::status::Status;
pub use self::network::Network;
pub use self::friend::{ Friend, FriendManage };
pub use self::address::{ PublicKey, Address };
pub use self::chat::Chat;
pub use self::custom::Packet;
pub use self::events::{ Event, Listen };

#[cfg(feature = "groupchat")]
pub use self::group::{ Group, GroupCreate };

#[cfg(feature = "groupchat")]
pub use self::peer::Peer;

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

    pub fn address(&self) -> Address {
        Address::from(out!( get
            out <- vec_with!(vars::TOX_ADDRESS_SIZE),
            ffi::tox_self_get_address(self.core, out.as_mut_ptr())
        ))
    }

    pub fn secretkey(&self) -> Vec<u8> {
        out!( get
            out <- vec_with!(vars::TOX_SECRET_KEY_SIZE),
            ffi::tox_self_get_secret_key(self.core, out.as_mut_ptr())
        )
    }

    pub fn nospam(&self) -> u32 {
        unsafe { ffi::tox_self_get_nospam(self.core) }
    }

    pub fn set_name<S: AsRef<[u8]>>(&self, name: S) -> Result<(), error::InfoSetErr> {
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
    pub fn set_nospam(&self, nospam: u32) {
        unsafe { ffi::tox_self_set_nospam(self.core, nospam) }
    }
    pub fn set_status(&self, status: status::UserStatus) {
        unsafe { ffi::tox_self_set_status(self.core, status) }
    }
    pub fn set_status_message<S: AsRef<[u8]>>(&self, message: S) -> Result<(), error::InfoSetErr> {
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
}

impl Drop for Tox {
    fn drop(&mut self) {
        unsafe { ffi::tox_kill(self.core); }
    }
}
