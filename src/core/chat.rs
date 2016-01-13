use super::{ ffi, error, Friend };
pub use super::ffi::TOX_MESSAGE_TYPE as MessageType;


pub type MessageID = u32;

pub trait Chat {
    fn send<S: AsRef<[u8]>>(&mut self, ty: MessageType, message: S) -> Result<MessageID, error::SendMessageErr>;
    fn say<S: AsRef<[u8]>>(&mut self, message: S) -> Result<MessageID, error::SendMessageErr>;
    fn action<S: AsRef<[u8]>>(&mut self, message: S) -> Result<MessageID, error::SendMessageErr>;
}

impl Chat for Friend {
    fn send<S: AsRef<[u8]>>(&mut self, ty: MessageType, message: S) -> Result<MessageID, error::SendMessageErr> {
        let message = message.as_ref();
        out!( err
            err,
            ffi::tox_friend_send_message(
                self.core,
                self.number,
                ty,
                message.as_ptr(),
                message.len(),
                &mut err
            )
        ).map_err(|err| error::SendMessageErr::from(err))
    }

    fn say<S: AsRef<[u8]>>(&mut self, message: S) -> Result<MessageID, error::SendMessageErr> {
        self.send(MessageType::NORMAL, message)
    }

    fn action<S: AsRef<[u8]>>(&mut self, message: S) -> Result<MessageID, error::SendMessageErr> {
        self.send(MessageType::ACTION, message)
    }
}
