use jsonwebtoken::{DecodingKey, EncodingKey};

pub struct Keys {
    pub decoding: DecodingKey,
    pub encoding: EncodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            decoding: DecodingKey::from_secret(secret),
            encoding: EncodingKey::from_secret(secret),
        }
    }
}