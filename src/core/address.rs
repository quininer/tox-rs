use std::str::FromStr;
use rustc_serialize::hex::FromHex;
use super::vars::{ TOX_PUBLIC_KEY_SIZE, TOX_ADDRESS_SIZE };
use super::error;


macro_rules! to_slice {
    ( $vec:expr, $len:expr ) => {{
        let mut out = [0u8; $len];
        for (x, &y) in $vec.iter().enumerate() {
            out[x] = y;
        }
        out
    }};
}

#[derive(PartialEq, Clone, Debug)]
pub struct PublicKey {
    raw: [u8; TOX_PUBLIC_KEY_SIZE]
}

#[derive(PartialEq, Clone, Debug)]
pub struct Address {
    publickey: PublicKey,
    nospam: [u8; 4],
    checksum: [u8; 2]
}

impl PublicKey {
    pub fn new(v: &[u8]) -> PublicKey {
        PublicKey { raw: to_slice!(v, TOX_PUBLIC_KEY_SIZE) }
    }
}

impl Address {
    pub fn new(v: &[u8]) -> Address {
        let (pk, nnc) = v.split_at(TOX_PUBLIC_KEY_SIZE);
        let (nospam, checksum) = nnc.split_at(4);
        Address {
            publickey: PublicKey { raw: to_slice!(pk, TOX_PUBLIC_KEY_SIZE) },
            nospam: to_slice!(nospam, 4),
            checksum: to_slice!(checksum, 2)
        }
    }

    pub fn out(&self) -> Vec<u8> {
        vec![
            self.publickey.as_ref().to_vec(),
            self.nospam.to_vec(),
            self.checksum.to_vec()
        ].concat()
    }

    pub fn check(&self) -> bool {
        let mut check = [0u8; 2];
        for (i, &k) in self.publickey.as_ref().iter().enumerate() {
            check[i % 2] ^= k;
        }
        for i in 0..4 {
            check[(TOX_PUBLIC_KEY_SIZE + i) % 2] ^= self.nospam[i];
        }
        check == self.checksum
    }
}

/// ```
/// extern crate rustc_serialize;
/// extern crate tox;
/// use rustc_serialize::hex::ToHex;
/// use tox::core::PublicKey;
///
/// fn main() {
///     let hex = "EDF5A5BE8DFFC1DDFAACC71A0C0FCEEDE7BED4F3FBF9C54D502BE66A297DC374";
///     let pk: PublicKey = hex.parse().unwrap();
///     assert_eq!(pk.as_ref().to_hex().to_uppercase(), hex);
/// }
/// ```
impl FromStr for PublicKey {
    type Err = error::AddressParserErr;
    fn from_str(s: &str) -> Result<PublicKey, error::AddressParserErr> {
        let key = try!(s.from_hex());
        if key.len() == TOX_PUBLIC_KEY_SIZE {
            Ok(PublicKey { raw: to_slice!(key, TOX_PUBLIC_KEY_SIZE) })
        } else {
            Err(error::AddressParserErr::InvalidLength)
        }
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.raw
    }
}


/// ```
/// extern crate rustc_serialize;
/// extern crate tox;
/// use rustc_serialize::hex::ToHex;
/// use tox::core::vars::TOX_PUBLIC_KEY_SIZE;
/// use tox::core::Address;
///
/// fn main() {
///     let hex = "EDF5A5BE8DFFC1DDFAACC71A0C0FCEEDE7BED4F3FBF9C54D502BE66A297DC37469CDD2311170";
///     let address: Address = hex.parse().unwrap();
///     assert_eq!(
///         address.as_ref().as_ref().to_hex().to_uppercase(),
///         &hex[..TOX_PUBLIC_KEY_SIZE*2]
///     );
/// }
/// ```
impl FromStr for Address {
    type Err = error::AddressParserErr;
    fn from_str(s: &str) -> Result<Address, error::AddressParserErr> {
        if s.len() == TOX_ADDRESS_SIZE * 2 {
            let (pk, nnc) = s.split_at(TOX_PUBLIC_KEY_SIZE * 2);
            let (nospam, checksum) = nnc.split_at(4 * 2);
            let address = Address {
                publickey: try!(pk.parse()),
                nospam: to_slice!(try!(nospam.from_hex()), 4),
                checksum: to_slice!(try!(checksum.from_hex()), 2)
            };
            if address.check() {
                Ok(address)
            } else {
                Err(error::AddressParserErr::InvalidChecksum)
            }
        } else {
            Err(error::AddressParserErr::InvalidLength)
        }
    }
}

impl AsRef<PublicKey> for Address {
    fn as_ref(&self) -> &PublicKey {
        &self.publickey
    }
}
