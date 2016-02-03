mod ffi;
pub mod error;


pub const PASS_KEY_LENGTH: usize = 32;
pub const PASS_SALT_LENGTH: usize = 32;
pub const PASS_ENCRYPTION_EXTRA_LENGTH: usize = 80;


/// Determining whether the data is encrypted.
pub fn is_encrypted(data: &[u8]) -> bool {
    unsafe { ffi::tox_is_data_encrypted(data.as_ptr()) }
}


#[derive(Clone, Debug)]
pub struct ToxPassKey {
    passkey: ffi::TOX_PASS_KEY
}

/// ToxPassKey, Symmetric encryption local files.
///
/// # Examples
///
/// ```
/// use tox::encryptsave::ToxPassKey;
///
/// let passphrase = b"tox-rs";
/// let data = b"Tox on Rust, oh shift..";
///
/// let ciphertext = ToxPassKey::new(passphrase).unwrap()
///     .encrypt(data).unwrap();
/// let plaintext = ToxPassKey::from(passphrase, &ciphertext).unwrap()
///     .decrypt(&ciphertext).unwrap();
///
/// assert_eq!(data.to_vec(), plaintext);
/// ```
impl ToxPassKey {
    /// Generate ToxPassKey, using a random salt.
    pub fn new(passphrase: &[u8]) -> Result<ToxPassKey, error::KeyDerivationErr> {
        out!( out
            out, err,
            ffi::tox_derive_key_from_pass(
                passphrase.as_ptr(),
                passphrase.len(),
                &mut out,
                &mut err
            )
        ).map(|r| ToxPassKey { passkey: r })
    }

    /// Generate Tox PassKey, read salt from the data.
    pub fn from(passphrase: &[u8], data: &[u8]) -> Result<ToxPassKey, error::KeyDerivationErr> {
        ToxPassKey::with(passphrase, &unsafe {
            let mut salt = vec_with!(PASS_KEY_LENGTH);
            ffi::tox_get_salt(data.as_ptr(), salt.as_mut_ptr());
            salt
        })
    }

    /// Generate ToxPassKey, using the specified salt.
    pub fn with(passphrase: &[u8], salt: &[u8]) -> Result<ToxPassKey, error::KeyDerivationErr> {
        out!( out
            out, err,
            ffi::tox_derive_key_with_salt(
                passphrase.as_ptr(),
                passphrase.len(),
                salt.as_ptr(),
                &mut out,
                &mut err
            )
        ).map(|r| ToxPassKey { passkey: r })
    }

    /// encryption
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, error::EncrytpionErr> {
        out!( out
            out <- vec_with!(data.len() + PASS_ENCRYPTION_EXTRA_LENGTH),
            err,
            ffi::tox_pass_key_encrypt(
                data.as_ptr(),
                data.len(),
                &self.passkey,
                out.as_mut_ptr(),
                &mut err
            )
        )
    }

    /// decryption
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, error::DecryptionErr> {
        out!( out
            out <- vec_with!(data.len() - PASS_ENCRYPTION_EXTRA_LENGTH),
            err,
            ffi::tox_pass_key_decrypt(
                data.as_ptr(),
                data.len(),
                &self.passkey,
                out.as_mut_ptr(),
                &mut err
            )
        )
    }
}


/// use passphrase encryption
pub fn pass_encrypt<S: AsRef<[u8]>>(passphrase: S, data: &[u8]) -> Result<Vec<u8>, error::EncrytpionErr> {
    let passphrase = passphrase.as_ref();
    out!( out
        out <- vec_with!(data.len() + PASS_ENCRYPTION_EXTRA_LENGTH),
        err,
        ffi::tox_pass_encrypt(
            data.as_ptr(),
            data.len(),
            passphrase.as_ptr(),
            passphrase.len(),
            out.as_mut_ptr(),
            &mut err
        )
    )
}

/// use passphrase decryption
pub fn pass_decrypt<S: AsRef<[u8]>>(passphrase: S, data: &[u8]) -> Result<Vec<u8>, error::DecryptionErr> {
    let passphrase = passphrase.as_ref();
    out!( out
        out <- vec_with!(data.len() - PASS_ENCRYPTION_EXTRA_LENGTH),
        err,
        ffi::tox_pass_decrypt(
            data.as_ptr(),
            data.len(),
            passphrase.as_ptr(),
            passphrase.len(),
            out.as_mut_ptr(),
            &mut err
        )
    )
}
