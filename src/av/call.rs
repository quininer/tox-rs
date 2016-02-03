use super::{ ffi, error, FriendAv };
use_as! {
    TOXAV_CALL_CONTROL as CallControl,
    TOXAV_FRIEND_CALL_STATE as FriendCallState
}

pub trait Call {
    fn call(&self, audio_bitrate: u32, video_bitrate: u32) -> Result<(), error::CallErr>;
    fn answer(&self, audio_bitrate: u32, video_bitrate: u32) -> Result<(), error::AnswerErr>;
    fn control(&self, control: CallControl) -> Result<(), error::CallControlErr>;
    fn set_bitrate(&self, audio_bitrate: i32, video_bitrate: i32) -> Result<(), error::BitRateSetErr>;
    fn send_audio(&self, pcm: &[i16], channels: u8, sampling_rate: u32) -> Result<(), error::SendFrameErr>;
    fn send_video(&self, width: u16, height: u16, y: &[u8], u: &[u8], v: &[u8]) -> Result<(), error::SendFrameErr>;
}

impl Call for FriendAv {
    fn call(&self, audio_bitrate: u32, video_bitrate: u32) -> Result<(), error::CallErr> {
        out!( bool
            err,
            ffi::toxav_call(
                self.core,
                self.number,
                audio_bitrate,
                video_bitrate,
                &mut err
            )
        )
    }

    fn answer(&self, audio_bitrate: u32, video_bitrate: u32) -> Result<(), error::AnswerErr> {
        out!( bool
            err,
            ffi::toxav_answer(
                self.core,
                self.number,
                audio_bitrate,
                video_bitrate,
                &mut err
            )
        )
    }

    fn control(&self, control: CallControl) -> Result<(), error::CallControlErr> {
        out!( bool
            err,
            ffi::toxav_call_control(
                self.core,
                self.number,
                control,
                &mut err
            )
        )
    }

    fn set_bitrate(&self, audio_bitrate: i32, video_bitrate: i32) -> Result<(), error::BitRateSetErr> {
        out!( bool
            err,
            ffi::toxav_bit_rate_set(
                self.core,
                self.number,
                audio_bitrate,
                video_bitrate,
                &mut err
            )
        )
    }

    fn send_audio(&self, pcm: &[i16], channels: u8, sampling_rate: u32) -> Result<(), error::SendFrameErr> {
        out!( bool
            err,
            ffi::toxav_audio_send_frame(
                self.core,
                self.number,
                pcm.as_ptr(),
                pcm.len(),
                channels,
                sampling_rate,
                &mut err
            )
        )
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
        )
    }
}
