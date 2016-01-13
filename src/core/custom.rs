use super::{ ffi, error };
use super::Friend;


pub trait Packet {
    fn send_lossy<S: AsRef<[u8]>>(&mut self, data: S) -> Result<(), error::CustomPacketErr>;
    fn send_lossless<S: AsRef<[u8]>>(&mut self, data: S) -> Result<(), error::CustomPacketErr>;
}

impl Packet for Friend {
    fn send_lossy<S: AsRef<[u8]>>(&mut self, data: S) -> Result<(), error::CustomPacketErr> {
        let data = data.as_ref();
        out!( bool
            err,
            ffi::tox_friend_send_lossy_packet(
                self.core,
                self.number,
                data.as_ptr(),
                data.len(),
                &mut err
            )
        )
    }

    fn send_lossless<S: AsRef<[u8]>>(&mut self, data: S) -> Result<(), error::CustomPacketErr> {
        let data = data.as_ref();
        out!( bool
            err,
            ffi::tox_friend_send_lossless_packet(
                self.core,
                self.number,
                data.as_ptr(),
                data.len(),
                &mut err
            )
        )
    }
}
