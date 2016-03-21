use super::{
    ffi, error,
    Tox, Friend
};
use_as!{
    TOX_GROUP_PRIVACY_STATE as GroupPrivacyState,
    TOX_GROUP_ROLE as GroupRole,
    TOX_GROUP_JOIN_FAIL as GroupJoinFail,
    TOX_GROUP_MOD_EVENT as GroupModEvent
}

pub struct Group {
    pub core: *mut ffi::Tox,
    pub number: u32
}

impl Group {
    pub fn from(core: *mut ffi::Tox, number: u32) -> Group {
        Group { core: core, number: number }
    }

    pub fn reconnect(&self) -> Result<(), error::GroupReconnectErr> {
        out!(bool
            err,
            ffi::tox_group_reconnect(
                self.core,
                self.number,
                &mut err
            )
        )
    }

    pub fn leave(&self) -> Result<(), error::GroupLeaveErr> {
        out!(bool
            err,
            ffi::tox_group_leave(
                self.core,
                self.number,
                &mut err
            )
        )
    }
}

pub trait GroupCreate {
    fn create_group(&self, name: &str, state: GroupPrivacyState) -> Group;
    fn join(&self, groupid: &[u8], password: &[u8]) -> Group;
}

impl GroupCreate for Tox {
    fn create_group(&self, name: &[u8], state: GroupPrivacyState) -> Group {
        out!(err
            err,
            ffi::tox_group_new(
                self.core,
                state,
                name.as_ptr(),
                name.len(),
                &mut err
            )
        ).map(|r| Group::from(self.core, r))
    }
    fn join(&self, groupid: &[u8], password: &[u8]) -> Group {
        out!(err
            err,
            ffi::tox_group_join(
                self.core,
                groupid.as_ptr(),
                password.as_ptr(),
                &mut err
            )
        ).map(|r| Group::from(self.core, r))
    }
}


