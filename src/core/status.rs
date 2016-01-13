use super::{ ffi, Tox, error, vars, Friend, PublicKey, Address };
pub use super::ffi::{
    TOX_USER_STATUS as UserStatus,
    TOX_CONNECTION as Connection
};


pub trait Status {
    fn name(&self) -> Result<Vec<u8>, error::QueryFriendErr>;
    fn address(&self) -> Result<Address, ()>;
    fn publickey(&self) -> Result<PublicKey, error::GetFriendPKErr>;
    fn nospam(&self) -> Result<u32, ()>;
    fn status(&self) -> Result<UserStatus, error::QueryFriendErr>;
    fn status_message(&self) -> Result<Vec<u8>, error::QueryFriendErr>;
    fn connection_status(&self) -> Result<Connection, error::QueryFriendErr>;
}

impl Status for Tox {
    fn name(&self) -> Result<Vec<u8>, error::QueryFriendErr> {
        Ok(out!( get
            out <- vec_with!(ffi::tox_self_get_name_size(self.core)),
            ffi::tox_self_get_name(self.core, out.as_mut_ptr())
        ))
    }

    fn address(&self) -> Result<Address, ()> {
        Ok(out!( get
            out <- vec_with!(vars::TOX_ADDRESS_SIZE),
            ffi::tox_self_get_address(self.core, out.as_mut_ptr())
        )).map(|r| Address::from(r))
    }

    fn publickey(&self) -> Result<PublicKey, error::GetFriendPKErr> {
        Ok(out!( get
            out <- vec_with!(vars::TOX_PUBLIC_KEY_SIZE),
            ffi::tox_self_get_public_key(self.core, out.as_mut_ptr())
        )).map(|r| PublicKey::from(r))
    }

    fn nospam(&self) -> Result<u32, ()> {
        Ok(unsafe { ffi::tox_self_get_nospam(self.core) })
    }

    fn status(&self) -> Result<UserStatus, error::QueryFriendErr> {
        Ok(unsafe { ffi::tox_self_get_status(self.core) })
    }

    fn status_message(&self) -> Result<Vec<u8>, error::QueryFriendErr> {
        Ok(out!( get
            out <- vec_with!(ffi::tox_self_get_status_message_size(self.core)),
            ffi::tox_self_get_status_message(self.core, out.as_mut_ptr())
        ))
    }

    fn connection_status(&self) -> Result<Connection, error::QueryFriendErr> {
        Ok(unsafe { ffi::tox_self_get_connection_status(self.core) })
    }
}


impl Status for Friend {
    fn name(&self) -> Result<Vec<u8>, error::QueryFriendErr> {
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
        )
    }

    fn address(&self) -> Result<Address, ()> {
        unimplemented!()
    }

    fn publickey(&self) -> Result<PublicKey, error::GetFriendPKErr> {
        out!( out
            out <- vec_with!(vars::TOX_PUBLIC_KEY_SIZE),
            err,
            ffi::tox_friend_get_public_key(
                self.core,
                self.number,
                out.as_mut_ptr(),
                &mut err
            )
        ).map(|r| PublicKey::from(r))
    }

    fn nospam(&self) -> Result<u32, ()> {
        unimplemented!()
    }

    fn status(&self) -> Result<UserStatus, error::QueryFriendErr> {
        out!( err
            err,
            ffi::tox_friend_get_status(
                self.core,
                self.number,
                &mut err
            )
        )
    }

    fn status_message(&self) -> Result<Vec<u8>, error::QueryFriendErr> {
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
        )
    }

    fn connection_status(&self) -> Result<Connection, error::QueryFriendErr> {
        out!( err
            err,
            ffi::tox_friend_get_connection_status(
                self.core,
                self.number,
                &mut err
            )
        )
    }
}
