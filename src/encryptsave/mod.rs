pub mod ffi;


pub const PASS_KEY_LENGTH: usize = 32;
pub const PASS_SALT_LENGTH: usize = 32;
pub const PASS_ENCRYPTION_EXTRA_LENGTH: usize = 80;

/// Get out & err.
macro_rules! get_out {
    ( $out:ident <- $rexpr:expr, $err:ident, $r:expr ) => {
        unsafe {
            let mut $out = $rexpr;
            let mut $err = ::std::mem::uninitialized();
            if $r != 0 {
                Ok($out)
            } else {
                Err($err)
            }
        }
    };
    ( $out:ident, $err:ident, $r:expr ) => {
        get_out!(
            $out <- ::std::mem::uninitialized(),
            $err,
            $r
        )
    }
}


/// Determining whether the data is encrypted.
pub fn is_encrypted(data: &[u8]) -> bool {
    unsafe { ffi::tox_is_data_encrypted(data.as_ptr()) != 0 }
}


#[derive(Clone)]
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
/// let data = b"Tox on Rust, use mio.";
///
/// let ciphertext = ToxPassKey::new(passphrase).ok().unwrap()
///     .encrypt(data).ok().unwrap();
/// let plaintext = ToxPassKey::from(passphrase, &ciphertext).ok().unwrap()
///     .decrypt(&ciphertext).ok().unwrap();
///
/// assert_eq!(
///     String::from_utf8_lossy(data),
///     String::from_utf8_lossy(&plaintext)
/// );
/// ```
#[allow(unused_mut)]
impl ToxPassKey {
    /// Generate ToxPassKey, using a random salt.
    pub fn new(passphrase: &[u8]) -> Result<ToxPassKey, ffi::TOX_ERR_KEY_DERIVATION>  {
        let out = try!(get_out!(
            out,
            err,
            ffi::tox_derive_key_from_pass(
                passphrase.as_ptr(),
                passphrase.len(),
                &mut out,
                &mut err
            )
        ));

        Ok(ToxPassKey { passkey: out })
    }

    /// Generate Tox PassKey, read salt from the data.
    pub fn from(passphrase: &[u8], data: &[u8]) -> Result<ToxPassKey, ffi::TOX_ERR_KEY_DERIVATION> {
        ToxPassKey::with(passphrase, unsafe {
            let mut salt = Vec::with_capacity(PASS_SALT_LENGTH);
            salt.set_len(PASS_SALT_LENGTH);
            ffi::tox_get_salt(data.as_ptr(), salt.as_mut_ptr());
            salt
        })
    }

    /// Generate ToxPassKey, using the specified salt.
    pub fn with(passphrase: &[u8], salt: Vec<u8>) -> Result<ToxPassKey, ffi::TOX_ERR_KEY_DERIVATION> {
        let out = try!(get_out!(
            out,
            err,
            ffi::tox_derive_key_with_salt(
                passphrase.as_ptr(),
                passphrase.len(),
                salt.as_ptr(),
                &mut out,
                &mut err
            )
        ));

        Ok(ToxPassKey { passkey: out })
    }

    /// encryption
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, ffi::TOX_ERR_ENCRYPTION> {
        get_out!(
            out <- {
                let len = data.len() + PASS_ENCRYPTION_EXTRA_LENGTH;
                let mut out = Vec::with_capacity(len);
                out.set_len(len);
                out
            },
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
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, ffi::TOX_ERR_DECRYPTION> {
        get_out!(
            out <- {
                let len = data.len() - PASS_ENCRYPTION_EXTRA_LENGTH;
                let mut out = Vec::with_capacity(len);
                out.set_len(len);
                out
            },
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
pub fn pass_encrypt(passphrase: &[u8], data: &[u8]) -> Result<Vec<u8>, ffi::TOX_ERR_ENCRYPTION> {
    get_out!(
        out <- {
            let len = data.len() + PASS_ENCRYPTION_EXTRA_LENGTH;
            let mut out = Vec::with_capacity(len);
            out.set_len(len);
            out
        },
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
pub fn pass_decrypt(passphrase: &[u8], data: &[u8]) -> Result<Vec<u8>, ffi::TOX_ERR_DECRYPTION> {
    get_out!(
        out <- {
            let len = data.len() - PASS_ENCRYPTION_EXTRA_LENGTH;
            let mut out = Vec::with_capacity(len);
            out.set_len(len);
            out
        },
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
