use std::ffi::{ CString, NulError };
use super::*;


#[derive(Copy, Clone)]
pub struct ToxOptions {
    pub opts: ffi::Struct_Tox_Options
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
        self.opts.ipv6_enabled = enable as u8;
        self
    }

    pub fn udp(mut self, enable: bool) -> ToxOptions {
        self.opts.udp_enabled = enable as u8;
        self
    }

    pub fn proxy(mut self, ptype: ffi::TOX_PROXY_TYPE, host: &str, port: u16) -> Result<ToxOptions, NulError> {
        self.opts.proxy_type = ptype;
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
        self.opts.savedata_type = ffi::TOX_SAVEDATA_TYPE_TOX_SAVE;
        self.opts.savedata_data = data.as_ptr();
        self.opts.savedata_length = data.len();
        self
    }

    pub fn generate(self) -> Result<Tox, ffi::TOX_ERR_NEW> {
        Tox::new(self)
    }
}
