use rustc_serialize::hex::FromHexError;
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
    TOX_ERR_BOOTSTRAP as BootstrapErr,
    TOX_ERR_FRIEND_SEND_MESSAGE as SendMessageFriendErr,
    TOX_ERR_FRIEND_CUSTOM_PACKET as CustomPacketErr
};


#[derive(Debug)]
pub enum GetStatusErr {
    Group,
    Query(QueryFriendErr),
    GetPK(GetFriendPKErr)
}

impl From<QueryFriendErr> for GetStatusErr {
    fn from(err: QueryFriendErr) -> GetStatusErr {
        GetStatusErr::Query(err)
    }
}

impl From<GetFriendPKErr> for GetStatusErr {
    fn from(err: GetFriendPKErr) -> GetStatusErr {
        GetStatusErr::GetPK(err)
    }
}


#[derive(Debug)]
pub enum AddressParserErr {
    InvalidLength,
    InvalidChecksum,
    HexError(FromHexError)
}

impl From<FromHexError> for AddressParserErr {
    fn from(err: FromHexError) -> AddressParserErr {
        AddressParserErr::HexError(err)
    }
}


#[derive(Debug)]
pub enum SendMessageErr {
    Friend(SendMessageFriendErr),
    Group
}

impl From<SendMessageFriendErr> for SendMessageErr {
    fn from(err: SendMessageFriendErr) -> SendMessageErr {
        SendMessageErr::Friend(err)
    }
}
