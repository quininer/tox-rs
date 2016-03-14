use std::slice;
use std::mem::{ transmute, size_of };
use libc::*;
use ::core::{ Tox, Friend, Group, Peer };
use super::{ ffi };


pub type AvGroupCallback = Fn(Group, Peer, &[i16], u32, u8, u32);

pub trait AvGroupCreate {
    fn create_group_av(&self, cb: Box<AvGroupCallback>) -> Result<Group, ()>;
    fn join_av(&self, friend: &Friend, data: &[u8], cb: Box<AvGroupCallback>) -> Result<Group, ()>;
}

impl AvGroupCreate for Tox {
    fn create_group_av(&self, cb: Box<AvGroupCallback>) -> Result<Group, ()> {
        match unsafe { ffi::toxav_add_av_groupchat(
            transmute(self.core),
            on_group_av,
            transmute(&cb)
        ) } {
            -1 => Err(()),
            num => Ok(Group::from(self.core, num))
        }
    }
    fn join_av(&self, friend: &Friend, data: &[u8], cb: Box<AvGroupCallback>) -> Result<Group, ()> {
        match unsafe { ffi::toxav_join_av_groupchat(
            transmute(self.core),
            friend.number as int32_t,
            data.as_ptr(),
            data.len() as uint16_t,
            on_group_av,
            transmute(&cb)
        ) } {
            -1 => Err(()),
            num => Ok(Group::from(self.core, num))
        }
    }
}

extern "C" fn on_group_av(
    core: *mut c_void,
    group_number: c_int,
    peer_number: c_int,
    pcm: *const int16_t,
    samples: c_uint,
    channels: uint8_t,
    sample_rate: c_uint,
    cb: *mut c_void
) {
    unsafe {
        let group = Group::from(transmute(core), group_number);
        let peer = Peer::from(&group, peer_number);
        let callback: &Box<AvGroupCallback> = transmute(cb);
        callback(
            group,
            peer,
            slice::from_raw_parts(pcm, samples as usize * channels as usize * size_of::<int16_t>()),
            samples,
            channels,
            sample_rate
        );
    }
}
