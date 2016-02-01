use super::{
    ffi, error, vars,
    Group, PublicKey,
    Status
};
use super::status::{ Connection, UserStatus };

#[derive(Clone, Debug)]
pub struct Peer {
    group: Group,
    number: i32
}

impl Peer {
    pub fn from(group: &Group, number: i32) -> Peer {
        Peer { group: group.clone(), number: number }
    }

    pub fn is_ours(&self) -> bool {
        unsafe { ffi::tox_group_peernumber_is_ours(
            self.group.core,
            self.group.number,
            self.number
        ) != 0 }
    }
}

impl Status for Peer {
    fn name(&self) -> Result<Vec<u8>, error::GetStatusErr> {
        let mut name = unsafe { vec_with!(vars::TOX_MAX_NAME_LENGTH) };
        match unsafe { ffi::tox_group_peername(
            self.group.core,
            self.group.number,
            self.number,
            name.as_mut_ptr()
        ) } {
            -1 => Err(error::GetStatusErr::Group),
            _ => Ok(name)
        }
    }
    fn publickey(&self) -> Result<PublicKey, error::GetStatusErr> {
        let mut pk = unsafe { vec_with!(vars::TOX_PUBLIC_KEY_SIZE) };
        match unsafe { ffi::tox_group_peer_pubkey(
            self.group.core,
            self.group.number,
            self.number,
            pk.as_mut_ptr()
        ) } {
            -1 => Err(error::GetStatusErr::Group),
            _ => Ok(PublicKey::from(pk))
        }
    }
    fn status(&self) -> Result<UserStatus, error::GetStatusErr> {
        unimplemented!()
    }
    fn status_message(&self) -> Result<Vec<u8>, error::GetStatusErr> {
        unimplemented!()
    }
    fn connection_status(&self) -> Result<Connection, error::GetStatusErr> {
        unimplemented!()
    }
}
