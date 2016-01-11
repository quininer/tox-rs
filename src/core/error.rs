pub use super::ffi::{
    TOX_ERR_NEW as NewErr,
    TOX_ERR_FRIEND_ADD as AddFriendErr,
    TOX_ERR_SET_INFO as InfoSetErr,
    TOX_ERR_FRIEND_DELETE as DelFriendErr,
    TOX_ERR_FRIEND_QUERY as QueryFriendErr,
    TOX_ERR_FRIEND_GET_PUBLIC_KEY as GetFriendPKErr,
    TOX_ERR_FRIEND_BY_PUBLIC_KEY as PKGetFriendErr,
    TOX_ERR_FRIEND_GET_LAST_ONLINE as GetFriendLastErr,
    TOX_ERR_SET_TYPING as TypingSetErr,
    TOX_ERR_GET_PORT as GetPortErr,
    TOX_ERR_BOOTSTRAP as BootstrapErr
};
