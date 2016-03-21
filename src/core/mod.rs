mod ffi;
pub mod error;
pub mod vars;
pub mod options;
pub mod status;
pub mod chat;
pub mod file;
mod friend;
mod network;
mod custom;
mod events;

#[cfg(feature = "groupchat")]
pub mod group;

#[cfg(feature = "groupchat")]
mod peer;

#[cfg(feature = "new-groupchat")]
mod newgcgroup;

#[cfg(feature = "new-groupchat")]
mod newgcpeer;

pub use self::options::ToxOptions;
pub use self::status::{ Status, SetStatus };
pub use self::network::Network;
pub use self::friend::{ Friend, FriendManage };
pub use self::chat::Chat;
pub use self::custom::Packet;
pub use self::events::{ Event, Listen };
pub use self::file::File;

#[cfg(feature = "groupchat")]
pub use self::group::Group;

#[cfg(feature = "groupchat")]
pub use self::peer::Peer;

#[cfg(feature = "new-groupchat")]
pub mod group {
    pub use super::newgcgroup::*;
}

#[cfg(feature = "new-groupchat")]
mod peer {
    pub use super::newgcpeer::*;
}

#[cfg(feature = "new-groupchat")]
pub use self::group::Group;


/// Tox.
#[derive(Clone, Debug)]
pub struct Tox {
    pub core: *mut ffi::Tox
}

impl Tox {
    /// from ToxOptions create Tox.
    pub fn new(opts: ToxOptions) -> Result<Tox, error::NewErr> {
        out!(err err, ffi::tox_new(&opts.opts, &mut err))
            .map(Tox::from)
    }

    /// from raw ptr create Tox.
    pub fn from(core: *mut ffi::Tox) -> Tox {
        Tox { core: core }
    }

    /// Get Tox Profile data.
    pub fn save(&self) -> Vec<u8> {
        out!( get
            out <- vec_with!(ffi::tox_get_savedata_size(self.core)),
            ffi::tox_get_savedata(self.core, out.as_mut_ptr())
        )
    }

    /// Get Address.
    pub fn address(&self) -> ::address::Address {
        out!( get
            out <- vec_with!(vars::TOX_ADDRESS_SIZE),
            ffi::tox_self_get_address(self.core, out.as_mut_ptr())
        ).into()
    }

    /// Get SecretKey.
    pub fn secretkey(&self) -> Vec<u8> {
        out!( get
            out <- vec_with!(vars::TOX_SECRET_KEY_SIZE),
            ffi::tox_self_get_secret_key(self.core, out.as_mut_ptr())
        )
    }

    /// Get Nospam code.
    pub fn nospam(&self) -> u32 {
        unsafe { ffi::tox_self_get_nospam(self.core) }
    }

    /// Set Nospam code.
    pub fn set_nospam(&self, nospam: u32) {
        unsafe { ffi::tox_self_set_nospam(self.core, nospam) }
    }
    /// Set Status Message.
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


pub fn version_major() -> usize {
    unsafe { ffi::tox_version_major() as usize }
}

pub fn version_minor() -> usize {
    unsafe { ffi::tox_version_minor() as usize }
}

pub fn version_patch() -> usize {
    unsafe { ffi::tox_version_patch() as usize }
}

pub fn version_is_compatible(major: usize, minor: usize, patch: usize) -> bool {
    unsafe { ffi::tox_version_is_compatible(
        major as ::libc::uint32_t,
        minor as ::libc::uint32_t,
        patch as ::libc::uint32_t
    ) }
}
