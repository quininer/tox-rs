//! ffigen generate.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use libc::*;

pub type tox_self_connection_status_cb = extern "C" fn(
    tox: *mut Tox,
    connection_status: TOX_CONNECTION,
    user_data: *mut c_void,
) -> ();
pub type tox_friend_name_cb = extern "C" fn(
    tox: *mut Tox,
    friend_number: uint32_t,
    name: *const uint8_t,
    length: size_t,
    user_data: *mut c_void,
) -> ();
pub type tox_friend_status_message_cb = extern "C" fn(
    tox: *mut Tox,
    friend_number: uint32_t,
    message: *const uint8_t,
    length: size_t,
    user_data: *mut c_void,
) -> ();
pub type tox_friend_status_cb = extern "C" fn(
    tox: *mut Tox,
    friend_number: uint32_t,
    status: TOX_USER_STATUS,
    user_data: *mut c_void,
) -> ();
pub type tox_friend_connection_status_cb = extern "C" fn(
    tox: *mut Tox,
    friend_number: uint32_t,
    connection_status: TOX_CONNECTION,
    user_data: *mut c_void,
) -> ();
pub type tox_friend_typing_cb = extern "C" fn(
    tox: *mut Tox,
    friend_number: uint32_t,
    is_typing: bool,
    user_data: *mut c_void,
) -> ();
pub type tox_friend_read_receipt_cb = extern "C" fn(
    tox: *mut Tox,
    friend_number: uint32_t,
    message_id: uint32_t,
    user_data: *mut c_void,
) -> ();
pub type tox_friend_request_cb = extern "C" fn(
    tox: *mut Tox,
    public_key: *const uint8_t,
    message: *const uint8_t,
    length: size_t,
    user_data: *mut c_void,
) -> ();
pub type tox_friend_message_cb = extern "C" fn(
    tox: *mut Tox,
    friend_number: uint32_t,
    type_: TOX_MESSAGE_TYPE,
    message: *const uint8_t,
    length: size_t,
    user_data: *mut c_void,
) -> ();
pub type tox_file_recv_control_cb = extern "C" fn(
    tox: *mut Tox,
    friend_number: uint32_t,
    file_number: uint32_t,
    control: TOX_FILE_CONTROL,
    user_data: *mut c_void,
) -> ();
pub type tox_file_chunk_request_cb = extern "C" fn(
    tox: *mut Tox,
    friend_number: uint32_t,
    file_number: uint32_t,
    position: uint64_t,
    length: size_t,
    user_data: *mut c_void,
) -> ();
pub type tox_file_recv_cb = extern "C" fn(
    tox: *mut Tox,
    friend_number: uint32_t,
    file_number: uint32_t,
    kind: uint32_t,
    file_size: uint64_t,
    filename: *const uint8_t,
    filename_length: size_t,
    user_data: *mut c_void,
) -> ();
pub type tox_file_recv_chunk_cb = extern "C" fn(
    tox: *mut Tox,
    friend_number: uint32_t,
    file_number: uint32_t,
    position: uint64_t,
    data: *const uint8_t,
    length: size_t,
    user_data: *mut c_void,
) -> ();
pub type tox_friend_lossy_packet_cb = extern "C" fn(
    tox: *mut Tox,
    friend_number: uint32_t,
    data: *const uint8_t,
    length: size_t,
    user_data: *mut c_void,
) -> ();
pub type tox_friend_lossless_packet_cb = extern "C" fn(
    tox: *mut Tox,
    friend_number: uint32_t,
    data: *const uint8_t,
    length: size_t,
    user_data: *mut c_void,
) -> ();

pub enum Tox {}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_USER_STATUS {
    NONE = 0,
    AWAY = 1,
    BUSY = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_MESSAGE_TYPE {
    NORMAL = 0,
    ACTION = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_PROXY_TYPE {
    NONE = 0,
    HTTP = 1,
    SOCKS5 = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_SAVEDATA_TYPE {
    NONE = 0,
    TOX_SAVE = 1,
    SECRET_KEY = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Tox_Options {
    pub ipv6_enabled: bool,
    pub udp_enabled: bool,
    pub proxy_type: TOX_PROXY_TYPE,
    pub proxy_host: *const c_char,
    pub proxy_port: uint16_t,
    pub start_port: uint16_t,
    pub end_port: uint16_t,
    pub tcp_port: uint16_t,
    pub savedata_type: TOX_SAVEDATA_TYPE,
    pub savedata_data: *const uint8_t,
    pub savedata_length: size_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_OPTIONS_NEW {
    OK = 0,
    MALLOC = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_NEW {
    OK = 0,
    NULL = 1,
    MALLOC = 2,
    PORT_ALLOC = 3,
    PROXY_BAD_TYPE = 4,
    PROXY_BAD_HOST = 5,
    PROXY_BAD_PORT = 6,
    PROXY_NOT_FOUND = 7,
    LOAD_ENCRYPTED = 8,
    LOAD_BAD_FORMAT = 9,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_BOOTSTRAP {
    OK = 0,
    NULL = 1,
    BAD_HOST = 2,
    BAD_PORT = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_CONNECTION {
    NONE = 0,
    TCP = 1,
    UDP = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_SET_INFO {
    OK = 0,
    NULL = 1,
    TOO_LONG = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_FRIEND_ADD {
    OK = 0,
    NULL = 1,
    TOO_LONG = 2,
    NO_MESSAGE = 3,
    OWN_KEY = 4,
    ALREADY_SENT = 5,
    BAD_CHECKSUM = 6,
    SET_NEW_NOSPAM = 7,
    MALLOC = 8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_FRIEND_DELETE {
    OK = 0,
    FRIEND_NOT_FOUND = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_FRIEND_BY_PUBLIC_KEY {
    OK = 0,
    NULL = 1,
    NOT_FOUND = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_FRIEND_GET_PUBLIC_KEY {
    OK = 0,
    FRIEND_NOT_FOUND = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_FRIEND_GET_LAST_ONLINE {
    OK = 0,
    FRIEND_NOT_FOUND = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_FRIEND_QUERY {
    OK = 0,
    NULL = 1,
    FRIEND_NOT_FOUND = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_SET_TYPING {
    OK = 0,
    FRIEND_NOT_FOUND = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_FRIEND_SEND_MESSAGE {
    OK = 0,
    NULL = 1,
    FRIEND_NOT_FOUND = 2,
    FRIEND_NOT_CONNECTED = 3,
    SENDQ = 4,
    TOO_LONG = 5,
    EMPTY = 6,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_FILE_KIND {
    DATA = 0,
    AVATAR = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_FILE_CONTROL {
    RESUME = 0,
    PAUSE = 1,
    CANCEL = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_FILE_CONTROL {
    OK = 0,
    FRIEND_NOT_FOUND = 1,
    FRIEND_NOT_CONNECTED = 2,
    NOT_FOUND = 3,
    NOT_PAUSED = 4,
    DENIED = 5,
    ALREADY_PAUSED = 6,
    SENDQ = 7,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_FILE_SEEK {
    OK = 0,
    FRIEND_NOT_FOUND = 1,
    FRIEND_NOT_CONNECTED = 2,
    NOT_FOUND = 3,
    DENIED = 4,
    INVALID_POSITION = 5,
    SENDQ = 6,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_FILE_GET {
    OK = 0,
    NULL = 1,
    FRIEND_NOT_FOUND = 2,
    NOT_FOUND = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_FILE_SEND {
    OK = 0,
    NULL = 1,
    FRIEND_NOT_FOUND = 2,
    FRIEND_NOT_CONNECTED = 3,
    NAME_TOO_LONG = 4,
    TOO_MANY = 5,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_FILE_SEND_CHUNK {
    OK = 0,
    NULL = 1,
    FRIEND_NOT_FOUND = 2,
    FRIEND_NOT_CONNECTED = 3,
    NOT_FOUND = 4,
    NOT_TRANSFERRING = 5,
    INVALID_LENGTH = 6,
    SENDQ = 7,
    WRONG_POSITION = 8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_FRIEND_CUSTOM_PACKET {
    OK = 0,
    NULL = 1,
    FRIEND_NOT_FOUND = 2,
    FRIEND_NOT_CONNECTED = 3,
    INVALID = 4,
    EMPTY = 5,
    TOO_LONG = 6,
    SENDQ = 7,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_GET_PORT {
    OK = 0,
    NOT_BOUND = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Unnamed339 {
    TEXT = 0,
    AV = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_CHAT_CHANGE {
    ADD = 0,
    DEL = 1,
    NAME = 2,
}

#[link(name="toxcore")]
extern "C" {
    pub fn tox_version_major(
    ) -> uint32_t;
    pub fn tox_version_minor(
    ) -> uint32_t;
    pub fn tox_version_patch(
    ) -> uint32_t;
    pub fn tox_version_is_compatible(
        major: uint32_t,
        minor: uint32_t,
        patch: uint32_t,
    ) -> bool;
    pub fn tox_options_default(
        options: *mut Tox_Options,
    ) -> ();
    pub fn tox_options_new(
        error: *mut TOX_ERR_OPTIONS_NEW,
    ) -> *mut Tox_Options;
    pub fn tox_options_free(
        options: *mut Tox_Options,
    ) -> ();
    pub fn tox_new(
        options: *const Tox_Options,
        error: *mut TOX_ERR_NEW,
    ) -> *mut Tox;
    pub fn tox_kill(
        tox: *mut Tox,
    ) -> ();
    pub fn tox_get_savedata_size(
        tox: *const Tox,
    ) -> size_t;
    pub fn tox_get_savedata(
        tox: *const Tox,
        savedata: *mut uint8_t,
    ) -> ();
    pub fn tox_bootstrap(
        tox: *mut Tox,
        address: *const c_char,
        port: uint16_t,
        public_key: *const uint8_t,
        error: *mut TOX_ERR_BOOTSTRAP,
    ) -> bool;
    pub fn tox_add_tcp_relay(
        tox: *mut Tox,
        address: *const c_char,
        port: uint16_t,
        public_key: *const uint8_t,
        error: *mut TOX_ERR_BOOTSTRAP,
    ) -> bool;
    pub fn tox_self_get_connection_status(
        tox: *const Tox,
    ) -> TOX_CONNECTION;
    pub fn tox_callback_self_connection_status(
        tox: *mut Tox,
        callback: tox_self_connection_status_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn tox_iteration_interval(
        tox: *const Tox,
    ) -> uint32_t;
    pub fn tox_iterate(
        tox: *mut Tox,
    ) -> ();
    pub fn tox_self_get_address(
        tox: *const Tox,
        address: *mut uint8_t,
    ) -> ();
    pub fn tox_self_set_nospam(
        tox: *mut Tox,
        nospam: uint32_t,
    ) -> ();
    pub fn tox_self_get_nospam(
        tox: *const Tox,
    ) -> uint32_t;
    pub fn tox_self_get_public_key(
        tox: *const Tox,
        public_key: *mut uint8_t,
    ) -> ();
    pub fn tox_self_get_secret_key(
        tox: *const Tox,
        secret_key: *mut uint8_t,
    ) -> ();
    pub fn tox_self_set_name(
        tox: *mut Tox,
        name: *const uint8_t,
        length: size_t,
        error: *mut TOX_ERR_SET_INFO,
    ) -> bool;
    pub fn tox_self_get_name_size(
        tox: *const Tox,
    ) -> size_t;
    pub fn tox_self_get_name(
        tox: *const Tox,
        name: *mut uint8_t,
    ) -> ();
    pub fn tox_self_set_status_message(
        tox: *mut Tox,
        status_message: *const uint8_t,
        length: size_t,
        error: *mut TOX_ERR_SET_INFO,
    ) -> bool;
    pub fn tox_self_get_status_message_size(
        tox: *const Tox,
    ) -> size_t;
    pub fn tox_self_get_status_message(
        tox: *const Tox,
        status_message: *mut uint8_t,
    ) -> ();
    pub fn tox_self_set_status(
        tox: *mut Tox,
        status: TOX_USER_STATUS,
    ) -> ();
    pub fn tox_self_get_status(
        tox: *const Tox,
    ) -> TOX_USER_STATUS;
    pub fn tox_friend_add(
        tox: *mut Tox,
        address: *const uint8_t,
        message: *const uint8_t,
        length: size_t,
        error: *mut TOX_ERR_FRIEND_ADD,
    ) -> uint32_t;
    pub fn tox_friend_add_norequest(
        tox: *mut Tox,
        public_key: *const uint8_t,
        error: *mut TOX_ERR_FRIEND_ADD,
    ) -> uint32_t;
    pub fn tox_friend_delete(
        tox: *mut Tox,
        friend_number: uint32_t,
        error: *mut TOX_ERR_FRIEND_DELETE,
    ) -> bool;
    pub fn tox_friend_by_public_key(
        tox: *const Tox,
        public_key: *const uint8_t,
        error: *mut TOX_ERR_FRIEND_BY_PUBLIC_KEY,
    ) -> uint32_t;
    pub fn tox_friend_exists(
        tox: *const Tox,
        friend_number: uint32_t,
    ) -> bool;
    pub fn tox_self_get_friend_list_size(
        tox: *const Tox,
    ) -> size_t;
    pub fn tox_self_get_friend_list(
        tox: *const Tox,
        friend_list: *mut uint32_t,
    ) -> ();
    pub fn tox_friend_get_public_key(
        tox: *const Tox,
        friend_number: uint32_t,
        public_key: *mut uint8_t,
        error: *mut TOX_ERR_FRIEND_GET_PUBLIC_KEY,
    ) -> bool;
    pub fn tox_friend_get_last_online(
        tox: *const Tox,
        friend_number: uint32_t,
        error: *mut TOX_ERR_FRIEND_GET_LAST_ONLINE,
    ) -> uint64_t;
    pub fn tox_friend_get_name_size(
        tox: *const Tox,
        friend_number: uint32_t,
        error: *mut TOX_ERR_FRIEND_QUERY,
    ) -> size_t;
    pub fn tox_friend_get_name(
        tox: *const Tox,
        friend_number: uint32_t,
        name: *mut uint8_t,
        error: *mut TOX_ERR_FRIEND_QUERY,
    ) -> bool;
    pub fn tox_callback_friend_name(
        tox: *mut Tox,
        callback: tox_friend_name_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn tox_friend_get_status_message_size(
        tox: *const Tox,
        friend_number: uint32_t,
        error: *mut TOX_ERR_FRIEND_QUERY,
    ) -> size_t;
    pub fn tox_friend_get_status_message(
        tox: *const Tox,
        friend_number: uint32_t,
        status_message: *mut uint8_t,
        error: *mut TOX_ERR_FRIEND_QUERY,
    ) -> bool;
    pub fn tox_callback_friend_status_message(
        tox: *mut Tox,
        callback: tox_friend_status_message_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn tox_friend_get_status(
        tox: *const Tox,
        friend_number: uint32_t,
        error: *mut TOX_ERR_FRIEND_QUERY,
    ) -> TOX_USER_STATUS;
    pub fn tox_callback_friend_status(
        tox: *mut Tox,
        callback: tox_friend_status_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn tox_friend_get_connection_status(
        tox: *const Tox,
        friend_number: uint32_t,
        error: *mut TOX_ERR_FRIEND_QUERY,
    ) -> TOX_CONNECTION;
    pub fn tox_callback_friend_connection_status(
        tox: *mut Tox,
        callback: tox_friend_connection_status_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn tox_friend_get_typing(
        tox: *const Tox,
        friend_number: uint32_t,
        error: *mut TOX_ERR_FRIEND_QUERY,
    ) -> bool;
    pub fn tox_callback_friend_typing(
        tox: *mut Tox,
        callback: tox_friend_typing_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn tox_self_set_typing(
        tox: *mut Tox,
        friend_number: uint32_t,
        typing: bool,
        error: *mut TOX_ERR_SET_TYPING,
    ) -> bool;
    pub fn tox_friend_send_message(
        tox: *mut Tox,
        friend_number: uint32_t,
        type_: TOX_MESSAGE_TYPE,
        message: *const uint8_t,
        length: size_t,
        error: *mut TOX_ERR_FRIEND_SEND_MESSAGE,
    ) -> uint32_t;
    pub fn tox_callback_friend_read_receipt(
        tox: *mut Tox,
        callback: tox_friend_read_receipt_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn tox_callback_friend_request(
        tox: *mut Tox,
        callback: tox_friend_request_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn tox_callback_friend_message(
        tox: *mut Tox,
        callback: tox_friend_message_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn tox_hash(
        hash: *mut uint8_t,
        data: *const uint8_t,
        length: size_t,
    ) -> bool;
    pub fn tox_file_control(
        tox: *mut Tox,
        friend_number: uint32_t,
        file_number: uint32_t,
        control: TOX_FILE_CONTROL,
        error: *mut TOX_ERR_FILE_CONTROL,
    ) -> bool;
    pub fn tox_callback_file_recv_control(
        tox: *mut Tox,
        callback: tox_file_recv_control_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn tox_file_seek(
        tox: *mut Tox,
        friend_number: uint32_t,
        file_number: uint32_t,
        position: uint64_t,
        error: *mut TOX_ERR_FILE_SEEK,
    ) -> bool;
    pub fn tox_file_get_file_id(
        tox: *const Tox,
        friend_number: uint32_t,
        file_number: uint32_t,
        file_id: *mut uint8_t,
        error: *mut TOX_ERR_FILE_GET,
    ) -> bool;
    pub fn tox_file_send(
        tox: *mut Tox,
        friend_number: uint32_t,
        kind: uint32_t,
        file_size: uint64_t,
        file_id: *const uint8_t,
        filename: *const uint8_t,
        filename_length: size_t,
        error: *mut TOX_ERR_FILE_SEND,
    ) -> uint32_t;
    pub fn tox_file_send_chunk(
        tox: *mut Tox,
        friend_number: uint32_t,
        file_number: uint32_t,
        position: uint64_t,
        data: *const uint8_t,
        length: size_t,
        error: *mut TOX_ERR_FILE_SEND_CHUNK,
    ) -> bool;
    pub fn tox_callback_file_chunk_request(
        tox: *mut Tox,
        callback: tox_file_chunk_request_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn tox_callback_file_recv(
        tox: *mut Tox,
        callback: tox_file_recv_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn tox_callback_file_recv_chunk(
        tox: *mut Tox,
        callback: tox_file_recv_chunk_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn tox_friend_send_lossy_packet(
        tox: *mut Tox,
        friend_number: uint32_t,
        data: *const uint8_t,
        length: size_t,
        error: *mut TOX_ERR_FRIEND_CUSTOM_PACKET,
    ) -> bool;
    pub fn tox_friend_send_lossless_packet(
        tox: *mut Tox,
        friend_number: uint32_t,
        data: *const uint8_t,
        length: size_t,
        error: *mut TOX_ERR_FRIEND_CUSTOM_PACKET,
    ) -> bool;
    pub fn tox_callback_friend_lossy_packet(
        tox: *mut Tox,
        callback: tox_friend_lossy_packet_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn tox_callback_friend_lossless_packet(
        tox: *mut Tox,
        callback: tox_friend_lossless_packet_cb,
        user_data: *mut c_void,
    ) -> ();
    pub fn tox_self_get_dht_id(
        tox: *const Tox,
        dht_id: *mut uint8_t,
    ) -> ();
    pub fn tox_self_get_udp_port(
        tox: *const Tox,
        error: *mut TOX_ERR_GET_PORT,
    ) -> uint16_t;
    pub fn tox_self_get_tcp_port(
        tox: *const Tox,
        error: *mut TOX_ERR_GET_PORT,
    ) -> uint16_t;
    pub fn tox_callback_group_invite(
        tox: *mut Tox,
        function: extern "C" fn(
            tox: *mut Tox,
            Unnamed839: int32_t,
            Unnamed841: uint8_t,
            Unnamed843: *const uint8_t,
            Unnamed845: uint16_t,
            Unnamed847: *mut c_void,
        ) -> (),
        userdata: *mut c_void,
    ) -> ();
    pub fn tox_callback_group_message(
        tox: *mut Tox,
        function: extern "C" fn(
            tox: *mut Tox,
            Unnamed855: c_int,
            Unnamed856: c_int,
            Unnamed857: *const uint8_t,
            Unnamed859: uint16_t,
            Unnamed861: *mut c_void,
        ) -> (),
        userdata: *mut c_void,
    ) -> ();
    pub fn tox_callback_group_action(
        tox: *mut Tox,
        function: extern "C" fn(
            tox: *mut Tox,
            Unnamed869: c_int,
            Unnamed870: c_int,
            Unnamed871: *const uint8_t,
            Unnamed873: uint16_t,
            Unnamed875: *mut c_void,
        ) -> (),
        userdata: *mut c_void,
    ) -> ();
    pub fn tox_callback_group_title(
        tox: *mut Tox,
        function: extern "C" fn(
            tox: *mut Tox,
            Unnamed883: c_int,
            Unnamed884: c_int,
            Unnamed885: *const uint8_t,
            Unnamed887: uint8_t,
            Unnamed889: *mut c_void,
        ) -> (),
        userdata: *mut c_void,
    ) -> ();
    pub fn tox_callback_group_namelist_change(
        tox: *mut Tox,
        function: extern "C" fn(
            tox: *mut Tox,
            Unnamed897: c_int,
            Unnamed898: c_int,
            Unnamed899: uint8_t,
            Unnamed901: *mut c_void,
        ) -> (),
        userdata: *mut c_void,
    ) -> ();
    pub fn tox_add_groupchat(
        tox: *mut Tox,
    ) -> c_int;
    pub fn tox_del_groupchat(
        tox: *mut Tox,
        groupnumber: c_int,
    ) -> c_int;
    pub fn tox_group_peername(
        tox: *const Tox,
        groupnumber: c_int,
        peernumber: c_int,
        name: *mut uint8_t,
    ) -> c_int;
    pub fn tox_group_peer_pubkey(
        tox: *const Tox,
        groupnumber: c_int,
        peernumber: c_int,
        public_key: *mut uint8_t,
    ) -> c_int;
    pub fn tox_invite_friend(
        tox: *mut Tox,
        friendnumber: int32_t,
        groupnumber: c_int,
    ) -> c_int;
    pub fn tox_join_groupchat(
        tox: *mut Tox,
        friendnumber: int32_t,
        data: *const uint8_t,
        length: uint16_t,
    ) -> c_int;
    pub fn tox_group_message_send(
        tox: *mut Tox,
        groupnumber: c_int,
        message: *const uint8_t,
        length: uint16_t,
    ) -> c_int;
    pub fn tox_group_action_send(
        tox: *mut Tox,
        groupnumber: c_int,
        action: *const uint8_t,
        length: uint16_t,
    ) -> c_int;
    pub fn tox_group_set_title(
        tox: *mut Tox,
        groupnumber: c_int,
        title: *const uint8_t,
        length: uint8_t,
    ) -> c_int;
    pub fn tox_group_get_title(
        tox: *mut Tox,
        groupnumber: c_int,
        title: *mut uint8_t,
        max_length: uint32_t,
    ) -> c_int;
    pub fn tox_group_peernumber_is_ours(
        tox: *const Tox,
        groupnumber: c_int,
        peernumber: c_int,
    ) -> c_uint;
    pub fn tox_group_number_peers(
        tox: *const Tox,
        groupnumber: c_int,
    ) -> c_int;
    pub fn tox_group_get_names(
        tox: *const Tox,
        groupnumber: c_int,
        names: *mut [uint8_t; 128],
        lengths: *mut uint16_t,
        length: uint16_t,
    ) -> c_int;
    pub fn tox_count_chatlist(
        tox: *const Tox,
    ) -> uint32_t;
    pub fn tox_get_chatlist(
        tox: *const Tox,
        out_list: *mut int32_t,
        list_size: uint32_t,
    ) -> uint32_t;
    pub fn tox_group_get_type(
        tox: *const Tox,
        groupnumber: c_int,
    ) -> c_int;
}