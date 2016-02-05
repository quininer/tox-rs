#[cfg(feature = "groupchat")]
use std::mem::transmute;

#[cfg(feature = "groupchat")]
use ::core::group::Group;

use super::{ ffi, error, AvFriend };

pub trait AvSend {
    /// Send audio frame.
    fn send_audio(&self, pcm: &[i16], sample_count: usize, channels: u8, sampling_rate: u32) -> Result<(), error::SendFrameErr>;
    /// Send video frame.
    fn send_video(&self, width: u16, height: u16, y: &[u8], u: &[u8], v: &[u8]) -> Result<(), error::SendFrameErr>;
}

impl AvSend for AvFriend {
    fn send_audio(&self, pcm: &[i16], sample_count: usize, channels: u8, sampling_rate: u32) -> Result<(), error::SendFrameErr> {
        out!( bool
            err,
            ffi::toxav_audio_send_frame(
                self.core,
                self.number,
                pcm.as_ptr(),
                sample_count,
                // pcm.len() / channels as usize,
                // sampling_rate as usize * 10 / 1000,
                channels,
                sampling_rate,
                &mut err
            )
        ).map_err(|err| err.into())
    }

    fn send_video(&self, width: u16, height: u16, y: &[u8], u: &[u8], v: &[u8]) -> Result<(), error::SendFrameErr> {
        out!( bool
            err,
            ffi::toxav_video_send_frame(
                self.core,
                self.number,
                width,
                height,
                y.as_ptr(),
                u.as_ptr(),
                v.as_ptr(),
                &mut err
            )
        ).map_err(|err| err.into())
    }
}


#[cfg(feature = "groupchat")]
impl AvSend for Group {
    fn send_audio(&self, pcm: &[i16], samples: usize, channels: u8, sampling_rate: u32) -> Result<(), error::SendFrameErr> {
        match unsafe { ffi::toxav_group_send_audio(
            transmute(self.core),
            self.number,
            pcm.as_ptr(),
            samples as u32,
            channels,
            sampling_rate
        ) } {
            0 => Ok(()),
            _ => Err(error::SendFrameErr::Group)
        }
    }

    #[allow(unused_variables)]
    fn send_video(&self, width: u16, height: u16, y: &[u8], u: &[u8], v: &[u8]) -> Result<(), error::SendFrameErr> {
        unimplemented!()
    }
}
