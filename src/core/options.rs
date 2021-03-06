use std::net::ToSocketAddrs;
use std::ffi::CString;
use ::utils::addr_to_string;
use super::ffi;
use_as! {
    TOX_SAVEDATA_TYPE as SavedataType,
    TOX_PROXY_TYPE as ProxyType
}


/// Tox Options.
#[derive(Clone, Debug)]
pub struct ToxOptions {
    pub opts: ffi::Tox_Options
}

impl Default for ToxOptions {
    /// Generate ToxOptions
    fn default() -> ToxOptions {
        ToxOptions {
            opts: out!(get out, ffi::tox_options_default(&mut out))
        }
    }
}

impl ToxOptions {
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
    pub fn proxy<A: ToSocketAddrs>(mut self, pty: ProxyType, addr: A) -> Result<ToxOptions, ::std::io::Error> {
        let (host, port) = try!(addr_to_string(addr));
        self.opts.proxy_type = pty;
        self.opts.proxy_host = unsafe { CString::from_vec_unchecked(host.into()).as_ptr() };
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
        if self.opts.savedata_type == SavedataType::NONE {
            self.opts.savedata_type = SavedataType::TOX_SAVE;
        }
        self.opts.savedata_data = data.as_ptr();
        self.opts.savedata_length = data.len();
        self
    }

    /// is secretkey.
    pub fn secretkey(mut self) -> ToxOptions {
        self.opts.savedata_type = SavedataType::SECRET_KEY;
        self
    }

    /// Generate Tox.
    pub fn generate(self) -> Result<super::Tox, super::error::NewErr> {
        super::Tox::new(self)
    }
}
