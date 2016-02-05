use super::{ ffi, error, AvFriend };
use_as! {
    TOXAV_CALL_CONTROL as CallControl,
    TOXAV_FRIEND_CALL_STATE as CallState
}

pub trait Call {
    /// Call.
    fn call(&self, audio_bitrate: u32, video_bitrate: u32) -> Result<(), error::CallErr>;
    /// Answer.
    fn answer(&self, audio_bitrate: u32, video_bitrate: u32) -> Result<(), error::AnswerErr>;
    /// Control Call.
    fn control(&self, control: CallControl) -> Result<(), error::CallControlErr>;
    /// Set Bit Rate.
    fn set_bitrate(&self, audio_bitrate: i32, video_bitrate: i32) -> Result<(), error::BitRateSetErr>;
}

impl Call for AvFriend {
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
}
