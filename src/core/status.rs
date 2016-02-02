use super::{ ffi, Tox, error, vars, Friend, PublicKey };
use_as! {
    TOX_USER_STATUS as UserStatus,
    TOX_CONNECTION as Connection
}


/// Get Status.
pub trait Status {
    /// Get Name.
    fn name(&self) -> Result<Vec<u8>, error::GetStatusErr>;
    /// Get Public Key.
    fn publickey(&self) -> Result<PublicKey, error::GetStatusErr>;
    /// Get Status.
    fn status(&self) -> Result<UserStatus, error::GetStatusErr>;
    /// Get Status Message.
    fn status_message(&self) -> Result<Vec<u8>, error::GetStatusErr>;
    /// Get Connection Status.
    fn connection_status(&self) -> Result<Connection, error::GetStatusErr>;
}

impl Status for Tox {
    fn name(&self) -> Result<Vec<u8>, error::GetStatusErr> {
        Ok(out!( get
            out <- vec_with!(ffi::tox_self_get_name_size(self.core)),
            ffi::tox_self_get_name(self.core, out.as_mut_ptr())
        ))
    }

    fn publickey(&self) -> Result<PublicKey, error::GetStatusErr> {
        Ok(out!( get
            out <- vec_with!(vars::TOX_PUBLIC_KEY_SIZE),
            ffi::tox_self_get_public_key(self.core, out.as_mut_ptr())
        )).map(|r| r.into())
    }

    fn status(&self) -> Result<UserStatus, error::GetStatusErr> {
        Ok(unsafe { ffi::tox_self_get_status(self.core) })
    }

    fn status_message(&self) -> Result<Vec<u8>, error::GetStatusErr> {
        Ok(out!( get
            out <- vec_with!(ffi::tox_self_get_status_message_size(self.core)),
            ffi::tox_self_get_status_message(self.core, out.as_mut_ptr())
        ))
    }

    fn connection_status(&self) -> Result<Connection, error::GetStatusErr> {
        Ok(unsafe { ffi::tox_self_get_connection_status(self.core) })
    }
}


impl Status for Friend {
    fn name(&self) -> Result<Vec<u8>, error::GetStatusErr> {
        let len = try!(out!( err
            err,
            ffi::tox_friend_get_name_size(
                self.core,
                self.number,
                &mut err
            )
        ));
        out!( out
            out <- vec_with!(len),
            err,
            ffi::tox_friend_get_name(
                self.core,
                self.number,
                out.as_mut_ptr(),
                &mut err
            )
        ).map_err(|err| err.into())
    }

    fn publickey(&self) -> Result<PublicKey, error::GetStatusErr> {
        out!( out
            out <- vec_with!(vars::TOX_PUBLIC_KEY_SIZE),
            err,
            ffi::tox_friend_get_public_key(
                self.core,
                self.number,
                out.as_mut_ptr(),
                &mut err
            )
        )
            .map(|r| r.into())
            .map_err(|err| err.into())
    }

    fn status(&self) -> Result<UserStatus, error::GetStatusErr> {
        out!( err
            err,
            ffi::tox_friend_get_status(
                self.core,
                self.number,
                &mut err
            )
        ).map_err(|err| err.into())
    }

    fn status_message(&self) -> Result<Vec<u8>, error::GetStatusErr> {
        let len = try!(out!( err
            err,
            ffi::tox_friend_get_status_message_size(
                self.core,
                self.number,
                &mut err
            )
        ));
        out!( out
            out <- vec_with!(len),
            err,
            ffi::tox_friend_get_status_message(
                self.core,
                self.number,
                out.as_mut_ptr(),
                &mut err
            )
        ).map_err(|err| err.into())
    }

    fn connection_status(&self) -> Result<Connection, error::GetStatusErr> {
        out!( err
            err,
            ffi::tox_friend_get_connection_status(
                self.core,
                self.number,
                &mut err
            )
        ).map_err(|err| err.into())
    }
}
