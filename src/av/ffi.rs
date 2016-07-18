//! ffigen generate.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use libc::*;

pub type toxav_call_cb = extern "C" fn(
    toxAV: *mut ToxAV,
    friend_number: uint32_t,
    audio_enabled: bool,
    video_enabled: bool,
    user_data: *mut c_void,
) -> ();
pub type toxav_call_state_cb = extern "C" fn(
    toxAV: *mut ToxAV,
    friend_number: uint32_t,
    state: uint32_t,
    user_data: *mut c_void,
) -> ();
pub type toxav_bit_rate_status_cb = extern "C" fn(
    toxAV: *mut ToxAV,
    friend_number: uint32_t,
    audio_bit_rate: uint32_t,
    video_bit_rate: uint32_t,
    user_data: *mut c_void,
) -> ();
pub type toxav_audio_receive_frame_cb = extern "C" fn(
    toxAV: *mut ToxAV,
    friend_number: uint32_t,
    pcm: *const int16_t,
    sample_count: size_t,
    channels: uint8_t,
    sampling_rate: uint32_t,
    user_data: *mut c_void,
) -> ();
pub type toxav_video_receive_frame_cb = extern "C" fn(
    toxAV: *mut ToxAV,
    friend_number: uint32_t,
    width: uint16_t,
    height: uint16_t,
    y: *const uint8_t,
    u: *const uint8_t,
    v: *const uint8_t,
    ystride: int32_t,
    ustride: int32_t,
    vstride: int32_t,
    user_data: *mut c_void,
) -> ();

pub enum Tox {}

pub enum ToxAV {}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOXAV_ERR_NEW {
    OK = 0,
    NULL = 1,
    MALLOC = 2,
    MULTIPLE = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOXAV_ERR_CALL {
    OK = 0,
    MALLOC = 1,
    SYNC = 2,
    FRIEND_NOT_FOUND = 3,
    FRIEND_NOT_CONNECTED = 4,
    FRIEND_ALREADY_IN_CALL = 5,
    INVALID_BIT_RATE = 6,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOXAV_ERR_ANSWER {
    OK = 0,
    SYNC = 1,
    CODEC_INITIALIZATION = 2,
    FRIEND_NOT_FOUND = 3,
    FRIEND_NOT_CALLING = 4,
    INVALID_BIT_RATE = 5,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOXAV_FRIEND_CALL_STATE {
    ERROR = 1,
    FINISHED = 2,
    SENDING_A = 4,
    SENDING_V = 8,
    ACCEPTING_A = 16,
    ACCEPTING_V = 32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOXAV_CALL_CONTROL {
    RESUME = 0,
    PAUSE = 1,
    CANCEL = 2,
    MUTE_AUDIO = 3,
    UNMUTE_AUDIO = 4,
    HIDE_VIDEO = 5,
    SHOW_VIDEO = 6,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOXAV_ERR_CALL_CONTROL {
    OK = 0,
    SYNC = 1,
    FRIEND_NOT_FOUND = 2,
    FRIEND_NOT_IN_CALL = 3,
    INVALID_TRANSITION = 4,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOXAV_ERR_BIT_RATE_SET {
    OK = 0,
    SYNC = 1,
    INVALID_AUDIO_BIT_RATE = 2,
    INVALID_VIDEO_BIT_RATE = 3,
    FRIEND_NOT_FOUND = 4,
    FRIEND_NOT_IN_CALL = 5,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOXAV_ERR_SEND_FRAME {
    OK = 0,
    NULL = 1,
    FRIEND_NOT_FOUND = 2,
    FRIEND_NOT_IN_CALL = 3,
    SYNC = 4,
    INVALID = 5,
    PAYLOAD_TYPE_DISABLED = 6,
    RTP_FAILED = 7,
}

#[link(name="toxav")]
extern "C" {
    pub fn toxav_version_major(
    ) -> uint32_t;
    pub fn toxav_version_minor(
    ) -> uint32_t;
    pub fn toxav_version_patch(
    ) -> uint32_t;
    pub fn toxav_version_is_compatible(
        major: uint32_t,
        minor: uint32_t,
        patch: uint32_t,
    ) -> bool;
    pub fn toxav_new(
        tox: *mut Tox,
        error: *mut TOXAV_ERR_NEW,
    ) -> *mut ToxAV;
    pub fn toxav_kill(
        toxAV: *mut ToxAV,
    ) -> ();
    pub fn toxav_get_tox(
        toxAV: *const ToxAV,
    ) -> *mut Tox;
    pub fn toxav_iteration_interval(
        toxAV: *const ToxAV,
    ) -> uint32_t;
    pub fn toxav_iterate(
        toxAV: *mut ToxAV,
    ) -> ();
    pub fn toxav_call(
        toxAV: *mut ToxAV,
        friend_number: uint32_t,
        audio_bit_rate: uint32_t,
        video_bit_rate: uint32_t,
        error: *mut TOXAV_ERR_CALL,
    ) -> bool;
    pub fn toxav_callback_call(
        toxAV: *mut ToxAV,
        callback: toxav_call_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn toxav_answer(
        toxAV: *mut ToxAV,
        friend_number: uint32_t,
        audio_bit_rate: uint32_t,
        video_bit_rate: uint32_t,
        error: *mut TOXAV_ERR_ANSWER,
    ) -> bool;
    pub fn toxav_callback_call_state(
        toxAV: *mut ToxAV,
        callback: toxav_call_state_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn toxav_call_control(
        toxAV: *mut ToxAV,
        friend_number: uint32_t,
        control: TOXAV_CALL_CONTROL,
        error: *mut TOXAV_ERR_CALL_CONTROL,
    ) -> bool;
    pub fn toxav_bit_rate_set(
        toxAV: *mut ToxAV,
        friend_number: uint32_t,
        audio_bit_rate: int32_t,
        video_bit_rate: int32_t,
        error: *mut TOXAV_ERR_BIT_RATE_SET,
    ) -> bool;
    pub fn toxav_callback_bit_rate_status(
        toxAV: *mut ToxAV,
        callback: toxav_bit_rate_status_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn toxav_audio_send_frame(
        toxAV: *mut ToxAV,
        friend_number: uint32_t,
        pcm: *const int16_t,
        sample_count: size_t,
        channels: uint8_t,
        sampling_rate: uint32_t,
        error: *mut TOXAV_ERR_SEND_FRAME,
    ) -> bool;
    pub fn toxav_video_send_frame(
        toxAV: *mut ToxAV,
        friend_number: uint32_t,
        width: uint16_t,
        height: uint16_t,
        y: *const uint8_t,
        u: *const uint8_t,
        v: *const uint8_t,
        error: *mut TOXAV_ERR_SEND_FRAME,
    ) -> bool;
    pub fn toxav_callback_audio_receive_frame(
        toxAV: *mut ToxAV,
        callback: toxav_audio_receive_frame_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn toxav_callback_video_receive_frame(
        toxAV: *mut ToxAV,
        callback: toxav_video_receive_frame_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn toxav_add_av_groupchat(
        tox: *mut Tox,
        audio_callback: extern "C" fn(
            Unnamed273: *mut c_void,
            Unnamed274: c_int,
            Unnamed275: c_int,
            Unnamed276: *const int16_t,
            Unnamed278: c_uint,
            Unnamed279: uint8_t,
            Unnamed281: c_uint,
            Unnamed282: *mut c_void,
        ) -> (),
        userdata: *mut c_void,
    ) -> c_int;
    pub fn toxav_join_av_groupchat(
        tox: *mut Tox,
        friendnumber: int32_t,
        data: *const uint8_t,
        length: uint16_t,
        audio_callback: extern "C" fn(
            Unnamed294: *mut c_void,
            Unnamed295: c_int,
            Unnamed296: c_int,
            Unnamed297: *const int16_t,
            Unnamed299: c_uint,
            Unnamed300: uint8_t,
            Unnamed302: c_uint,
            Unnamed303: *mut c_void,
        ) -> (),
        userdata: *mut c_void,
    ) -> c_int;
    pub fn toxav_group_send_audio(
        tox: *mut Tox,
        groupnumber: c_int,
        pcm: *const int16_t,
        samples: c_uint,
        channels: uint8_t,
        sample_rate: c_uint,
    ) -> c_int;
}