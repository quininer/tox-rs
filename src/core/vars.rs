pub const TOX_PUBLIC_KEY_SIZE: usize = 32;
pub const TOX_SECRET_KEY_SIZE: usize = 32;
// TOX_PUBLIC_KEY_SIZE + size_of::<::libc::uint32_t>() + size_of::<::libc::uint16_t>()
pub const TOX_ADDRESS_SIZE: usize = TOX_PUBLIC_KEY_SIZE + 4 + 2;
pub const TOX_MAX_NAME_LENGTH: usize = 128;
pub const TOX_MAX_STATUS_MESSAGE_LENGTH: usize = 1007;
pub const TOX_MAX_FRIEND_REQUEST_LENGTH: usize = 1016;
pub const TOX_MAX_MESSAGE_LENGTH: usize = 1372;
pub const TOX_MAX_CUSTOM_PACKET_SIZE: usize = 1373;
pub const TOX_HASH_LENGTH: usize = 32;
pub const TOX_FILE_ID_LENGTH: usize = 32;
pub const TOX_MAX_FILENAME_LENGTH: usize = 32;
