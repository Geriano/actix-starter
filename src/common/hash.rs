use std::fmt;

use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug)]
pub struct Hash([u8; 32]);

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&hex::encode(self.0))
    }
}

impl PartialEq for Hash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl From<Vec<u8>> for Hash {
    fn from(value: Vec<u8>) -> Self {
        let mut digest = [0u8; 32];

        digest.copy_from_slice(&value);

        Self::from(digest)
    }
}

impl From<[u8; 32]> for Hash {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

impl From<String> for Hash {
    fn from(value: String) -> Self {
        Self::from(hex::decode(value).unwrap())
    }
}

impl From<&String> for Hash {
    fn from(value: &String) -> Self {
        Self::from(hex::decode(value).unwrap())
    }
}

impl From<&str> for Hash {
    fn from(value: &str) -> Self {
        Self::from(hex::decode(value).unwrap())
    }
}

impl Hash {
    pub fn verify(self, other: Hash) -> bool {
        self == other
    }
}

pub fn make<S: AsRef<[u8]>, M: AsRef<[u8]>>(salt: S, message: M) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(salt);
    hasher.update(message);

    Hash::from(hasher.finalize().to_vec())
}

pub fn verify<S: AsRef<[u8]>, M: AsRef<[u8]>>(hashed: Hash, salt: S, message: M) -> bool {
    hashed.verify(make(salt, message))
}

#[cfg(test)]
pub mod test {
    #[test]
    pub async fn hash_must_be_equal() {
        use super::make;
        use uuid::Uuid;

        let salt = Uuid::new_v4();
        let message = "Hello World";

        assert!(make(salt, message).verify(make(salt, "Hello World")))
    }

    #[test]
    pub async fn hash_must_not_be_equal() {
        use super::make;
        use uuid::Uuid;

        let salt = Uuid::new_v4();
        let message = "Hello World";

        assert!(!make(salt, message).verify(make(salt, "Hello world")))
    }
}
