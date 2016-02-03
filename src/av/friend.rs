use super::{ ffi };


#[derive(Clone, Debug)]
pub struct FriendAv {
    pub core: *mut ffi::ToxAV,
    pub number: u32
}

impl FriendAv {
    pub fn from(core: *mut ffi::ToxAV, number: u32) -> FriendAv {
        FriendAv { core: core, number: number }
    }
}
