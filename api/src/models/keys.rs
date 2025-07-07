use jsonwebtoken::{DecodingKey, EncodingKey};

/// Container for JWT cryptographic keys used for signing and verifying tokens.
pub struct Keys {
    /// Key used for verifying incoming JWT tokens.
    pub decoding: DecodingKey,
    /// Key used for signing outgoing JWT tokens.
    pub encoding: EncodingKey,
}

impl Keys {
    /// Creates new symmetric JWT keys from a shared secret byte slice.
    pub fn new(secret: &[u8]) -> Self {
        Self {
            decoding: DecodingKey::from_secret(secret),
            encoding: EncodingKey::from_secret(secret),
        }
    }
}
