use_as! {
    TOXAV_ERR_NEW as NewAVErr,
    TOXAV_ERR_CALL as CallErr,
    TOXAV_ERR_ANSWER as AnswerErr,
    TOXAV_ERR_CALL_CONTROL as CallControlErr,
    TOXAV_ERR_BIT_RATE_SET as BitRateSetErr,
    TOXAV_ERR_SEND_FRAME as FriendSendFrameErr
}

#[derive(Debug)]
pub enum SendFrameErr {
    Friend(FriendSendFrameErr),
    Group
}

impl From<FriendSendFrameErr> for SendFrameErr {
    fn from(err: FriendSendFrameErr) -> SendFrameErr {
        SendFrameErr::Friend(err)
    }
}
