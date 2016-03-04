mod ffi;
mod send;
mod friend;
pub mod error;
pub mod toav;
pub mod events;
pub mod call;

#[cfg(feature = "groupchat")]
mod group;

use std::mem::transmute;
use ::core::Tox;
pub use self::events::AvEvent;
pub use self::friend::AvFriend;
pub use self::call::Call;
pub use self::send::AvSend;

#[cfg(feature = "groupchat")]
pub use self::group::{ AvGroupCreate, AvGroupCallback };


/// ToxAv.
#[derive(Clone, Debug)]
pub struct ToxAv {
    pub core: *mut ffi::ToxAV
}

impl ToxAv {
    pub fn new(tox: &Tox) -> Result<ToxAv, error::NewAVErr> {
        out!(err err, ffi::toxav_new(transmute(tox.core), &mut err))
            .map(|o| ToxAv::from(o))
    }

    pub fn from(core: *mut ffi::ToxAV) -> ToxAv {
        ToxAv { core: core }
    }
}

impl Drop for ToxAv {
    fn drop(&mut self) {
        unsafe { ffi::toxav_kill(self.core); }
    }
}
