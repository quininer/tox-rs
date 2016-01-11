use super::{ ffi, Tox, error, vars };

pub trait Status {
    fn name(&self) -> Result<Vec<u8>, error::QueryFriendErr>;
    fn address(&self) -> Result<Vec<u8>, ()>;
    fn publickey(&self) -> Result<Vec<u8>, error::GetFriendPKErr>;
    fn nospam(&self) -> Result<usize, ()>;
    fn status(&self) -> Result<vars::UserStatus, error::QueryFriendErr>;
    fn status_message(&self) -> Result<Vec<u8>, error::QueryFriendErr>;
    fn connection_status(&self) -> Result<vars::Connection, error::QueryFriendErr>;
}

impl Status for Tox {
    fn name(&self) -> Result<Vec<u8>, error::QueryFriendErr> {
        Ok(out!( get
            out <- vec_with!(ffi::tox_self_get_name_size(self.core)),
            ffi::tox_self_get_name(self.core, out.as_mut_ptr())
        ))
    }
    fn address(&self) -> Result<Vec<u8>, ()> {
        Ok(out!( get
            out <- vec_with!(vars::TOX_ADDRESS_SIZE),
            ffi::tox_self_get_address(self.core, out.as_mut_ptr())
        ))
    }
    fn publickey(&self) -> Result<Vec<u8>, error::GetFriendPKErr> {
        Ok(out!( get
            out <- vec_with!(vars::TOX_PUBLIC_KEY_SIZE),
            ffi::tox_self_get_public_key(self.core, out.as_mut_ptr())
        ))
    }
    fn nospam(&self) -> Result<usize, ()> {
        Ok(unsafe { ffi::tox_self_get_nospam(self.core) } as usize)
    }
    fn status(&self) -> Result<vars::UserStatus, error::QueryFriendErr> {
        Ok(unsafe { ffi::tox_self_get_status(self.core) })
    }
    fn status_message(&self) -> Result<Vec<u8>, error::QueryFriendErr> {
        Ok(out!( get
            out <- vec_with!(ffi::tox_self_get_status_message_size(self.core)),
            ffi::tox_self_get_status_message(self.core, out.as_mut_ptr())
        ))
    }
    fn connection_status(&self) -> Result<vars::Connection, error::QueryFriendErr> {
        Ok(unsafe { ffi::tox_self_get_connection_status(self.core) })
    }
}
