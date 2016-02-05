use ::core::{ Tox, Friend };
use super::{ ToxAv, AvFriend };


pub trait ToAv<A> {
    /// To av struct.
    fn to_av(&self, av: &ToxAv) -> A;
}

impl ToAv<AvFriend> for Friend {
    fn to_av(&self, av: &ToxAv) -> AvFriend {
        AvFriend::from(av.core, self.number)
    }
}

pub trait ToTox<T> {
    /// Tox tox struct.
    fn to_tox(&self, tox: &Tox) -> T;
}

// impl ToTox<Tox> for ToxAv {
//     fn to_tox(&self) -> Tox {
//         /// FIXME Segmentation fault.
//         Tox::from(unsafe { transmute(ffi::toxav_get_tox(self.core)) })
//     }
// }

impl ToTox<Friend> for AvFriend {
    fn to_tox(&self, tox: &Tox) -> Friend {
        Friend::from(
            tox.core,
            self.number
        )
    }
}
