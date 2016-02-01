use std::ffi::{ CString, NulError };
use super::ffi;
pub use super::ffi::{
    TOX_SAVEDATA_TYPE as SavedataType,
    TOX_PROXY_TYPE as ProxyType
};


/// Tox Options.
#[derive(Clone, Debug)]
pub struct ToxOptions {
    pub opts: ffi::Tox_Options
}

impl ToxOptions {
    /// Generate ToxOptions
    pub fn new() -> ToxOptions {
        ToxOptions {
            opts: out!(get out, ffi::tox_options_default(&mut out))
        }
    }

    /// Use IPv6.
    pub fn ipv6(mut self, enable: bool) -> ToxOptions {
        self.opts.ipv6_enabled = enable;
        self
    }

    /// Disable UDP.
    pub fn udp(mut self, disable: bool) -> ToxOptions {
        self.opts.udp_enabled = disable;
        self
    }

    /// Use Proxy.
    pub fn proxy(mut self, pty: ProxyType, host: &str, port: u16) -> Result<ToxOptions, NulError> {
        self.opts.proxy_type = pty;
        self.opts.proxy_host = try!(CString::new(host)).as_ptr();
        self.opts.proxy_port = port;
        Ok(self)
    }

    /// Specify Port.
    pub fn port(mut self, start: u16, end: u16, tcp: u16) -> ToxOptions {
        self.opts.start_port = start;
        self.opts.end_port = end;
        self.opts.tcp_port = tcp;
        self
    }

    /// Read Profile data.
    pub fn from(mut self, data: &[u8]) -> ToxOptions {
        self.opts.savedata_type = SavedataType::TOX_SAVE;
        self.opts.savedata_data = data.as_ptr();
        self.opts.savedata_length = data.len();
        self
    }

    /// Generate Tox.
    pub fn generate(self) -> Result<super::Tox, super::error::NewErr> {
        super::Tox::new(self)
    }
}
