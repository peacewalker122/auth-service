use std::fmt::Debug;

use anyhow::Context;
use hmac::Mac;

use super::{
    hmac::{HmacSha1, HmacSha256, HmacSha512, HMAC},
    util::encode_base32,
};

// TODO: Implement this as a MFA in our services.

pub struct Hotp {
    pub hash_function: HMAC,
    pub issuer: String,
    pub target_email: String,
    pub secret: String,
    pub otp_code_len: u8,
}

impl Debug for Hotp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hotp")
            .field("hash_function", &self.hash_function)
            .field("issuer", &self.issuer)
            .field("target_email", &self.target_email)
            .field("secret", &self.secret)
            .finish()
    }
}

impl Hotp {
    // constructor. by defaulut will made hash_function set for MACSHA256
    pub fn new(
        hash_function: Option<HMAC>,
        issuer: &str,
        target_email: &str,
        secret: &str,
        otp_code_len: u8,
    ) -> Self {
        Self {
            hash_function: hash_function.unwrap_or(HMAC::HMACSHA256),
            issuer: issuer.to_string(),
            target_email: target_email.to_string(),
            secret: secret.to_string(),
            otp_code_len,
        }
    }

    // return a 6 digit otp, implement RFC 4226
    pub fn hotp(&self, moving_factor: u64) -> anyhow::Result<u64> {
        let hash_msg = match self.hash_function {
            HMAC::HMACSHA256 => {
                let mut hash_msg = HmacSha256::new_from_slice(self.secret.as_bytes())
                    .context("failed to create hmac")?;

                let msg = moving_factor.to_be_bytes();

                hash_msg.update(&msg);

                hash_msg.finalize().into_bytes().to_vec()
            }
            HMAC::HMACSHA1 => {
                let mut hash_msg = HmacSha1::new_from_slice(self.secret.as_bytes())
                    .context("failed to create hmac")?;

                let msg = moving_factor.to_be_bytes();

                hash_msg.update(&msg);

                hash_msg.finalize().into_bytes().to_vec()
            }
            HMAC::HMACSHA512 => {
                let mut hash_msg = HmacSha512::new_from_slice(self.secret.as_bytes())
                    .context("failed to create hmac")?;
                let msg = moving_factor.to_be_bytes();

                hash_msg.update(&msg);

                hash_msg.finalize().into_bytes().to_vec()
            }
        };

        let otp_digit = dynamic_truncation(&hash_msg, Some(self.otp_code_len));

        Ok(otp_digit)
    }

    pub fn get_url(&self) -> String {
        // format: otpauth://totp/ACME%20Co:john.doe@email.com?secret=HXDMVJECJJWSRB3HWIZR4IFUGFTMXBOZ&issuer=ACME%20Co&algorithm=SHA1&digits=6&period=30
        // secret were encoded in base32 without padding
        // google use SHA1 algorithm as default.

        let secret = encode_base32(self.secret.as_bytes(), false);

        format!(
            "otpauth://totp/{issuer}:{email}?secret={secret}&issuer={issuer}&algorithm={algorithm}&digits=6&period=30",
            issuer = self.issuer,
            email = self.target_email,
            algorithm = self.hash_function.to_string(),
            secret = secret
        )
    }
}

fn dynamic_truncation(val: &[u8], digit: Option<u8>) -> u64 {
    let offset = (val[val.len() - 1] & 0xf) as usize;

    // extract the last 31 bits from the val. return 32 bit uint
    let value = (val[offset] as u32 & 0x7f) << 24
        | (val[offset + 1] as u32 & 0xff) << 16
        | (val[offset + 2] as u32 & 0xff) << 8
        | (val[offset + 3] as u32 & 0xff);

    value as u64 % pow(10 as u64, digit.unwrap_or(6) as u64)
}

fn pow(base: u64, exp: u64) -> u64 {
    let mut result = 1;
    for _ in 0..exp {
        result *= base;
    }
    result
}

#[cfg(test)]
mod test {
    use chrono::{TimeZone, Utc};

    use crate::pkg::{hmac::HMAC, hotp::Hotp};

    #[test]
    fn hotp_ok() {
        let val = Hotp::new(
            Some(HMAC::HMACSHA1),
            "authservice",
            "test@mail.com",
            "12345678901234567890",
            6,
        );

        let result = val.hotp(0).unwrap();

        assert_eq!(result, 755224);
    }

    // NOTE: Test vectors from http://tools.ietf.org/html/rfc6238#appendix-B
    // When we testing using the given secret on the url, we got the wrong result.
    // Because of the wrong secret, ref: https://www.rfc-editor.org/errata_search.php?rfc=6238
    #[test]
    fn totp_ok() {
        let val = Hotp::new(
            None,
            "authservice",
            "test@mail.com",
            "12345678901234567890123456789012",
            8,
        );

        // Get the current time in UTC
        let current_time = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 59).unwrap();

        // Calculate the Unix time (seconds since the Unix epoch)
        let mut unix_time = current_time.timestamp();

        // time step 30
        unix_time /= 30;
        dbg!(&unix_time);

        // we still got problem when follow the vector test from the rfc document.
        // expected problem could be int he msg packet, poor endian
        let otp = val.hotp(unix_time as u64).unwrap();

        assert_eq!(otp, 46119246);
    }
}
