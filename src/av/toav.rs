use ::core::{ Tox, Friend };
use super::{ ToxAv, FriendAv };


pub trait ToAv<A> {
    fn to_av(&self, av: &ToxAv) -> A;
}

impl ToAv<FriendAv> for Friend {
    fn to_av(&self, av: &ToxAv) -> FriendAv {
        FriendAv::from(av.core, self.number)
    }
}

pub trait ToTox<T> {
    fn to_tox(&self, tox: &Tox) -> T;
}

// impl ToTox<Tox> for ToxAv {
//     fn to_tox(&self) -> Tox {
//         /// FIXME Segmentation fault.
//         Tox::from(unsafe { transmute(ffi::toxav_get_tox(self.core)) })
//     }
// }

impl ToTox<Friend> for FriendAv {
    fn to_tox(&self, tox: &Tox) -> Friend {
        Friend::from(
            tox.core,
            self.number
        )
    }
}
