use ::address::PublicKey;
use super::{
    ffi, error, vars,
    Group,
    Status
};
use super::status::{ Connection, UserStatus };


/// GropuChat Peer.
#[derive(Clone, Debug)]
pub struct Peer {
    pub group: Group,
    pub number: i32
}

impl Peer {
    pub fn from(group: &Group, number: i32) -> Peer {
        Peer { group: group.clone(), number: number }
    }

    /// Peer is Self ?
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
        let len = unsafe { ffi::tox_group_number_peers(
            self.group.core,
            self.group.number
        ) };
        let mut names = unsafe { vec_with!(len as usize) };
        let mut lengths = unsafe { vec_with!(len as usize) };
        if self.number >= len || unsafe { ffi::tox_group_get_names(
            self.group.core,
            self.group.number,
            names.as_mut_ptr(),
            lengths.as_mut_ptr(),
            len as ::libc::uint16_t
        ) == -1 } {
            return Err(error::GetStatusErr::Group);
        };
        let name_len = lengths[self.number as usize];
        Ok(names[self.number as usize][..name_len as usize].into())
    }

    // fn name(&self) -> Result<Vec<u8>, error::GetStatusErr> {
    //     let mut name = unsafe { vec_with!(vars::TOX_MAX_NAME_LENGTH) };
    //     match unsafe { ffi::tox_group_peername(
    //         self.group.core,
    //         self.group.number,
    //         self.number,
    //         name.as_mut_ptr()
    //     ) } {
    //         -1 => Err(error::GetStatusErr::Group),
    //         _ => Ok(name)
    //     }
    // }

    fn publickey(&self) -> Result<PublicKey, error::GetStatusErr> {
        let mut pk = unsafe { vec_with!(vars::TOX_PUBLIC_KEY_SIZE) };
        match unsafe { ffi::tox_group_peer_pubkey(
            self.group.core,
            self.group.number,
            self.number,
            pk.as_mut_ptr()
        ) } {
            -1 => Err(error::GetStatusErr::Group),
            _ => Ok(pk.into())
        }
    }

    /// unimplemented, TODO New GroupChat.
    fn status(&self) -> Result<UserStatus, error::GetStatusErr> {
        unimplemented!()
    }

    /// unimplemented, TODO New GroupChat.
    fn status_message(&self) -> Result<Vec<u8>, error::GetStatusErr> {
        unimplemented!()
    }

    /// unimplemented, TODO New GroupChat.
    fn connection_status(&self) -> Result<Connection, error::GetStatusErr> {
        unimplemented!()
    }
}
