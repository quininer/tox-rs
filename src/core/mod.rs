pub mod ffi;
pub mod utils;

pub use super::core::utils::ToxOptions;


pub struct Tox {
    core: *mut ffi::Tox
}

impl Tox {
    pub fn new(opts: ToxOptions) -> Result<Tox, ffi::TOX_ERR_NEW> {
        Ok(Tox { core: try_err!(err, ffi::tox_new(&opts.opts, &mut err)) })
    }

    pub fn save(&self) -> Vec<u8> {
        unsafe {
            let len = ffi::tox_get_savedata_size(self.core);
            let mut data = Vec::with_capacity(len);
            data.set_len(len);
            ffi::tox_get_savedata(self.core, data.as_mut_ptr());
            data
        }
    }
}

impl Drop for Tox {
    fn drop(&mut self) {
        unsafe { ffi::tox_kill(self.core) }
    }
}
