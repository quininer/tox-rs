use std::io;
use_as! {
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
    TOX_ERR_FRIEND_CUSTOM_PACKET as CustomPacketErr,
    TOX_ERR_FILE_SEND as FileSendErr,
    TOX_ERR_FILE_SEND_CHUNK as FileChunkSendErr,
    TOX_ERR_FILE_CONTROL as FileControlErr,
    TOX_ERR_FILE_SEEK as FileSeekErr,
    TOX_ERR_FILE_GET as FileGetErr
}


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
pub enum SendMessageErr {
    Friend(SendMessageFriendErr),
    Group
}

impl From<SendMessageFriendErr> for SendMessageErr {
    fn from(err: SendMessageFriendErr) -> SendMessageErr {
        SendMessageErr::Friend(err)
    }
}

impl From<io::Error> for BootstrapErr {
    fn from(_: io::Error) -> BootstrapErr {
        BootstrapErr::BAD_HOST
    }
}
