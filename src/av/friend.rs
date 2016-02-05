use super::{ ffi };


#[derive(Clone, Debug)]
pub struct AvFriend {
    pub core: *mut ffi::ToxAV,
    pub number: u32
}

impl AvFriend {
    pub fn from(core: *mut ffi::ToxAV, number: u32) -> AvFriend {
        AvFriend { core: core, number: number }
    }
}
