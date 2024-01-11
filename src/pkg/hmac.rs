use hmac::Hmac;
use sha1::Sha1;
use sha2::{Sha256, Sha512};

pub type HmacSha256 = Hmac<Sha256>;
pub type HmacSha1 = Hmac<Sha1>;
pub type HmacSha512 = Hmac<Sha512>;

#[derive(Debug)]
pub enum HMAC {
    HMACSHA256,
    HMACSHA1,
    HMACSHA512,
}

impl ToString for HMAC {
    fn to_string(&self) -> String {
        match self {
            HMAC::HMACSHA256 => String::from("SHA256"),
            HMAC::HMACSHA1 => String::from("SHA1"),
            HMAC::HMACSHA512 => String::from("SHA512"),
        }
    }
}
