//! ffigen generate.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use libc::*;


pub enum Tox {}

pub enum Tox_Options {}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_KEY_DERIVATION {
    OK = 0,
    NULL = 1,
    FAILED = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_ENCRYPTION {
    OK = 0,
    NULL = 1,
    KEY_DERIVATION_FAILED = 2,
    FAILED = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TOX_ERR_DECRYPTION {
    OK = 0,
    NULL = 1,
    INVALID_LENGTH = 2,
    BAD_FORMAT = 3,
    KEY_DERIVATION_FAILED = 4,
    FAILED = 5,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TOX_PASS_KEY {
    pub salt: [uint8_t; 32],
    pub key: [uint8_t; 32],
}

#[link(name="toxencryptsave")]
extern "C" {
    pub fn toxes_version_major(
    ) -> uint32_t;
    pub fn toxes_version_minor(
    ) -> uint32_t;
    pub fn toxes_version_patch(
    ) -> uint32_t;
    pub fn toxes_version_is_compatible(
        major: uint32_t,
        minor: uint32_t,
        patch: uint32_t,
    ) -> bool;
    pub fn tox_pass_encrypt(
        data: *const uint8_t,
        data_len: size_t,
        passphrase: *const uint8_t,
        pplength: size_t,
        out: *mut uint8_t,
        error: *mut TOX_ERR_ENCRYPTION,
    ) -> bool;
    pub fn tox_pass_decrypt(
        data: *const uint8_t,
        length: size_t,
        passphrase: *const uint8_t,
        pplength: size_t,
        out: *mut uint8_t,
        error: *mut TOX_ERR_DECRYPTION,
    ) -> bool;
    pub fn tox_derive_key_from_pass(
        passphrase: *const uint8_t,
        pplength: size_t,
        out_key: *mut TOX_PASS_KEY,
        error: *mut TOX_ERR_KEY_DERIVATION,
    ) -> bool;
    pub fn tox_derive_key_with_salt(
        passphrase: *const uint8_t,
        pplength: size_t,
        salt: *const uint8_t,
        out_key: *mut TOX_PASS_KEY,
        error: *mut TOX_ERR_KEY_DERIVATION,
    ) -> bool;
    pub fn tox_get_salt(
        data: *const uint8_t,
        salt: *mut uint8_t,
    ) -> bool;
    pub fn tox_pass_key_encrypt(
        data: *const uint8_t,
        data_len: size_t,
        key: *const TOX_PASS_KEY,
        out: *mut uint8_t,
        error: *mut TOX_ERR_ENCRYPTION,
    ) -> bool;
    pub fn tox_pass_key_decrypt(
        data: *const uint8_t,
        length: size_t,
        key: *const TOX_PASS_KEY,
        out: *mut uint8_t,
        error: *mut TOX_ERR_DECRYPTION,
    ) -> bool;
    pub fn tox_is_data_encrypted(
        data: *const uint8_t,
    ) -> bool;
}