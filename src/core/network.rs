use std::ffi::CString;
use super::{ ffi, Tox, error, vars, PublicKey };


/// Network.
pub trait Network {
    fn bootstrap<S: AsRef<str>>(&self, ipaddress: S, port: u16, public_key: PublicKey) -> Result<(), error::BootstrapErr>;
    fn addtcprelay<S: AsRef<str>>(&self, ipaddress: S, port: u16, public_key: PublicKey) -> Result<(), error::BootstrapErr>;
    fn dhtid(&self) -> Vec<u8>;
    fn udpport(&self) -> Result<u16, error::GetPortErr>;
    fn tcpport(&self) -> Result<u16, error::GetPortErr>;
}

impl Network for Tox {
    fn bootstrap<S: AsRef<str>>(&self, ipaddress: S, port: u16, public_key: PublicKey) -> Result<(), error::BootstrapErr> {
        let ipaddress = ipaddress.as_ref();
        out!( bool
            err,
            ffi::tox_bootstrap(
                self.core,
                CString::from_vec_unchecked(ipaddress.bytes().collect()).as_ptr(),
                port,
                public_key.as_ref().as_ptr(),
                &mut err
            )
        )
    }

    fn addtcprelay<S: AsRef<str>>(&self, ipaddress: S, port: u16, public_key: PublicKey) -> Result<(), error::BootstrapErr> {
        let ipaddress = ipaddress.as_ref();
        out!( bool
            err,
            ffi::tox_add_tcp_relay(
                self.core,
                CString::from_vec_unchecked(ipaddress.bytes().collect()).as_ptr(),
                port,
                public_key.as_ref().as_ptr(),
                &mut err
            )
        )
    }

    fn dhtid(&self) -> Vec<u8> {
        out!( get
            out <- vec_with!(vars::TOX_PUBLIC_KEY_SIZE),
            ffi::tox_self_get_dht_id(self.core, out.as_mut_ptr())
        )
    }

    fn udpport(&self) -> Result<u16, error::GetPortErr> {
        out!( err
            err,
            ffi::tox_self_get_udp_port(
                self.core,
                &mut err
            )
        )
    }

    fn tcpport(&self) -> Result<u16, error::GetPortErr> {
        out!( err
            err,
            ffi::tox_self_get_tcp_port(
                self.core,
                &mut err
            )
        )
    }
}
