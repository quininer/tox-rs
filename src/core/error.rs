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

#[cfg(feature = "new-groupchat")]
use_as!{
    TOX_ERR_GROUP_NEW as GroupNewErr,
    TOX_ERR_GROUP_JOIN as GroupJoinErr,
    TOX_ERR_GROUP_RECONNECT as GroupReconnectErr,
    TOX_ERR_GROUP_LEAVE as GroupLeaveErr,
    TOX_ERR_GROUP_SELF_QUERY as GroupSelfQueryErr,
    TOX_ERR_GROUP_SELF_NAME_SET as GroupSelfNameSetErr,
    TOX_ERR_GROUP_SELF_STATUS_SET as GroupSelfStatusSetErr,
    TOX_ERR_GROUP_PEER_QUERY as GroupPeerQueryErr,
    TOX_ERR_GROUP_STATE_QUERIES as GroupStateQueriesErr,
    TOX_ERR_GROUP_TOPIC_SET as GroupTopicSetErr,
    TOX_ERR_GROUP_SEND_MESSAGE as GroupSendMessageErr,
    TOX_ERR_GROUP_SEND_PRIVATE_MESSAGE as GroupSendPrivateMessageErr,
    TOX_ERR_GROUP_SEND_CUSTOM_PACKET as GroupSendCustomPacketErr,
    TOX_ERR_GROUP_INVITE_FRIEND as GroupInviteFriendErr,
    TOX_ERR_GROUP_INVITE_ACCEPT as GroupInviteAcceptErr,
    TOX_ERR_GROUP_FOUNDER_SET_PASSWORD as GroupFounderSetPasswordErr,
    TOX_ERR_GROUP_FOUNDER_SET_PRIVACY_STATE as GroupFounderSetPrivacyStateErr,
    TOX_ERR_GROUP_FOUNDER_SET_PEER_LIMIT as GroupFounderSetPeerLimitErr,
    TOX_ERR_GROUP_TOGGLE_IGNORE as GroupToggleIgnoreErr,
    TOX_ERR_GROUP_MOD_SET_ROLE as GroupModSetRoleErr,
    TOX_ERR_GROUP_MOD_REMOVE_PEER as GroupModRemovePeerErr,
    TOX_ERR_GROUP_MOD_REMOVE_BAN as GroupModRemoveBanErr,
    TOX_ERR_GROUP_BAN_QUERY as GroupBanQueryErr
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
