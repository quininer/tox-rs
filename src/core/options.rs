use std::ffi::{ CString, NulError };
use super::ffi;
pub use super::ffi::{
    TOX_SAVEDATA_TYPE as SaveDataType,
    TOX_PROXY_TYPE as ProxyType
};


#[derive(Clone, Debug)]
pub struct ToxOptions {
    pub opts: ffi::Tox_Options
}

impl ToxOptions {
    /// Generate ToxOptions
    pub fn new() -> ToxOptions {
        ToxOptions { opts: unsafe {
            let mut opts = ::std::mem::uninitialized();
            ffi::tox_options_default(&mut opts);
            opts
        } }
    }

    pub fn ipv6(mut self, enable: bool) -> ToxOptions {
        self.opts.ipv6_enabled = enable;
        self
    }

    pub fn udp(mut self, enable: bool) -> ToxOptions {
        self.opts.udp_enabled = enable;
        self
    }

    pub fn proxy(mut self, pty: ProxyType, host: &str, port: u16) -> Result<ToxOptions, NulError> {
        self.opts.proxy_type = pty;
        self.opts.proxy_host = try!(CString::new(host)).as_ptr();
        self.opts.proxy_port = port;
        Ok(self)
    }

    pub fn port(mut self, start: u16, end: u16, tcp: u16) -> ToxOptions {
        self.opts.start_port = start;
        self.opts.end_port = end;
        self.opts.tcp_port = tcp;
        self
    }

    pub fn from(mut self, data: &[u8]) -> ToxOptions {
        self.opts.savedata_type = SaveDataType::TOX_SAVE;
        self.opts.savedata_data = data.as_ptr();
        self.opts.savedata_length = data.len();
        self
    }

    pub fn generate(self) -> Result<super::Tox, super::error::NewErr> {
        super::Tox::new(self)
    }
}
