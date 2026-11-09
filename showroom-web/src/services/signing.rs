use std::sync::Arc;

use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;

#[derive(Clone)]
pub struct Signer {
    secret: Arc<[u8]>,
}

impl Signer {
    pub fn new(secret: &str) -> Self {
        Self { secret: secret.as_bytes().into() }
    }

    fn mac(&self, purpose: &str, parts: &[&str]) -> Hmac<Sha256> {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.secret).expect("hmac accepts keys of any length");
        mac.update(purpose.as_bytes());
        for part in parts {
            mac.update(b"\0");
            mac.update(part.as_bytes());
        }
        mac
    }

    pub fn sign(&self, purpose: &str, parts: &[&str]) -> String {
        encode_hex(&self.mac(purpose, parts).finalize().into_bytes())
    }

    pub fn verify(&self, purpose: &str, parts: &[&str], signature: &str) -> bool {
        decode_hex(signature).is_some_and(|bytes| self.mac(purpose, parts).verify_slice(&bytes).is_ok())
    }

    pub fn issue(&self, purpose: &str, subject: &str, binding: &str, ttl: Duration) -> String {
        let expires = (Utc::now() + ttl).timestamp().to_string();
        let signature = self.sign(purpose, &[subject, &expires, binding]);
        format!("{}.{expires}.{signature}", encode_hex(subject.as_bytes()))
    }

    pub fn subject(token: &str) -> Option<String> {
        let (subject, _) = token.split_once('.')?;
        String::from_utf8(decode_hex(subject)?).ok()
    }

    pub fn open(&self, purpose: &str, token: &str, binding: &str) -> Option<String> {
        let mut parts = token.split('.');
        let (Some(subject), Some(expires), Some(signature), None) = (parts.next(), parts.next(), parts.next(), parts.next()) else {
            return None;
        };
        let subject = String::from_utf8(decode_hex(subject)?).ok()?;
        if expires.parse::<i64>().ok()? <= Utc::now().timestamp() {
            return None;
        }
        self.verify(purpose, &[&subject, expires, binding], signature).then_some(subject)
    }
}

fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn decode_hex(hex: &str) -> Option<Vec<u8>> {
    if !hex.len().is_multiple_of(2) || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    (0..hex.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&hex[index..index + 2], 16).ok())
        .collect()
}
