use std::sync::mpsc::{ channel, Sender, Receiver };
use std::time::Duration;
use std::mem::transmute;
use std::cmp::max;
use std::slice;
use libc::*;

use ::core::Listen;
use super::{ ffi, ToxAv, AvFriend };
use super::call::CallState;


#[derive(Clone, Debug)]
pub enum AvEvent {
    /// Friend Call, `[friendav, audio enable, video enable].`
    FriendCall(AvFriend, bool, bool),
    /// Friend Call State.
    FriendCallState(AvFriend, CallState),
    /// Friend Bitrate Status, `[friendav, audio bitrate, video bitrate].`
    FriendBitRateStatus(AvFriend, u32, u32),
    /// Friend Audio frame receive, `[friendav, pcm, sample_count, channels, sampling_rate].`
    FriendAudioFrameReceive(AvFriend, Vec<i16>, usize, u8, u32),
    /// Friend Video frame receive, `[friendav, width, height, y, u, v, ystride, ustride, vstride].`
    FriendVideoFrameReceive(AvFriend, u16, u16, Vec<u8>, Vec<u8>, Vec<u8>, i32, i32, i32)
}


impl Listen<AvEvent> for ToxAv {
    fn _interval(&self) -> Duration {
        Duration::from_millis(unsafe {
            ffi::toxav_iteration_interval(self.core) as u64
        })
    }
    fn _iterate(&mut self) {
        unsafe { ffi::toxav_iterate(self.core) }
    }
    fn iterate(&mut self) -> Receiver<AvEvent> {
        let (sender, receiver) = channel::<AvEvent>();

        unsafe {
            let tx: *mut c_void = transmute(Box::new(sender));

            callback!( (toxav, self.core, tx),
                call,
                call_state,
                bit_rate_status,
                audio_receive_frame,
                video_receive_frame
            );
        }

        receiver
    }
}


extern "C" fn on_call(
    core: *mut ffi::ToxAV,
    friend_number: uint32_t,
    audio_enabled: bool,
    video_enabled: bool,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<AvEvent> = transmute(tx);
        sender.send(AvEvent::FriendCall(
            AvFriend::from(core, friend_number),
            audio_enabled,
            video_enabled
        )).ok();
    }
}

extern "C" fn on_call_state(
    core: *mut ffi::ToxAV,
    friend_number: uint32_t,
    state: uint32_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<AvEvent> = transmute(tx);
        sender.send(AvEvent::FriendCallState(
            AvFriend::from(core, friend_number),
            transmute(state)
        )).ok();
    }
}

extern "C" fn on_bit_rate_status(
    core: *mut ffi::ToxAV,
    friend_number: uint32_t,
    audio_bitrate: uint32_t,
    video_bitrate: uint32_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<AvEvent> = transmute(tx);
        sender.send(AvEvent::FriendBitRateStatus(
            AvFriend::from(core, friend_number),
            audio_bitrate,
            video_bitrate
        )).ok();
    }
}

extern "C" fn on_audio_receive_frame(
    core: *mut ffi::ToxAV,
    friend_number: uint32_t,
    pcm: *const int16_t,
    sample_count: size_t,
    channels: uint8_t,
    sampling_rate: uint32_t,
    tx: *mut c_void
) {
    unsafe {
        let sender: &Sender<AvEvent> = transmute(tx);
        sender.send(AvEvent::FriendAudioFrameReceive(
            AvFriend::from(core, friend_number),
            slice::from_raw_parts(pcm, sample_count * channels as usize).into(),
            sample_count,
            channels,
            sampling_rate
        )).ok();
    }
}

extern "C" fn on_video_receive_frame(
    core: *mut ffi::ToxAV,
    friend_number: uint32_t,
    width: uint16_t,
    height: uint16_t,
    y: *const uint8_t,
    u: *const uint8_t,
    v: *const uint8_t,
    ystride: int32_t,
    ustride: int32_t,
    vstride: int32_t,
    tx: *mut c_void,
) {
    unsafe {
        let sender: &Sender<AvEvent> = transmute(tx);
        sender.send(AvEvent::FriendVideoFrameReceive(
            AvFriend::from(core, friend_number),
            width,
            height,
            slice::from_raw_parts(
                y,
                max(width as i32, ystride.abs()) as usize * height as usize
            ).into(),
            slice::from_raw_parts(
                u,
                max((width / 2) as i32, ustride.abs()) as usize * (height / 2) as usize
            ).into(),
            slice::from_raw_parts(
                v,
                max((width / 2) as i32, vstride.abs()) as usize * (height / 2) as usize
            ).into(),
            ystride,
            ustride,
            vstride
        )).ok();
    }
}
