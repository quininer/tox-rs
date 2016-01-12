use super::error;
pub use super::ffi::TOX_MESSAGE_TYPE as MessageType;


pub type MessageID = u32;

pub trait Chat {
    fn send<S: AsRef<[u8]>>(&mut self, ty: MessageType, message: S) -> Result<MessageID, error::SendMessageErr>;
    fn say<S: AsRef<[u8]>>(&mut self, message: S) -> Result<MessageID, error::SendMessageErr>;
    fn action<S: AsRef<[u8]>>(&mut self, message: S) -> Result<MessageID, error::SendMessageErr>;
}
