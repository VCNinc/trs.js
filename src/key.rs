use wasm_bindgen::prelude::*;

use crate::prelude::Vec;

use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;

/// A key pair
#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct KeyPair {
    pub public: PublicKey,
    pub private: PrivateKey
}

/// A public key
#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct PublicKey(pub(crate) RistrettoPoint);

impl PublicKey {
    /// Serialize this public key to 32 bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        let c = self.0.compress();
        c.as_bytes().to_vec()
    }

    // TODO: Make this more robust
    /// Deserialize this public key from 32 bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<PublicKey> {
        if bytes.len() != 32 {
            return None;
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(bytes);
        let c = CompressedRistretto(arr);
        c.decompress().map(|p| PublicKey(p))
    }
}

/// A private key
#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct PrivateKey(pub(crate) Scalar, pub(crate) RistrettoPoint);

impl PrivateKey {
    /// Serialize this private key to 64 bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        let privkey_bytes = self.0.as_bytes().to_vec();
        let pubkey_bytes = {
            let p = PublicKey(self.1.clone());
            p.as_bytes()
        };

        [privkey_bytes, pubkey_bytes].concat()
    }

    // TODO: Make more robust
    /// Deserialize this private key from 64 bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<PrivateKey> {
        if bytes.len() != 64 {
            return None;
        }
        let (scalar_bytes, pubkey_point_bytes) = bytes.split_at(32);

        let scalar = {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(scalar_bytes);
            Scalar::from_canonical_bytes(arr)?
        };
        let pubkey_point = {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(pubkey_point_bytes);
            let c = CompressedRistretto(arr);
            c.decompress()
        };

        pubkey_point.map(|p| PrivateKey(scalar, p))
    }
}

/// Generates a secure random keypair
#[wasm_bindgen]
pub fn gen_keypair() -> KeyPair {
    let mut rng = rand::thread_rng();
    let s = Scalar::random(&mut rng);
    let pubkey = PublicKey(&s * &RISTRETTO_BASEPOINT_POINT);
    let privkey = PrivateKey(s, pubkey.0.clone());

    return KeyPair {
        public: pubkey,
        private: privkey
    };
}

/// Serialize a public key
#[wasm_bindgen]
pub fn public_to_ascii(pubkey: &PublicKey) -> JsValue {
    let encoded: Vec<u8> = pubkey.as_bytes();
    return JsValue::from_str(&base64::encode(&encoded));
}

/// Deserialize a public key
#[wasm_bindgen]
pub fn ascii_to_public(pubkey: &JsValue) -> PublicKey {
    let encoded = base64::decode(pubkey.as_string().unwrap()).unwrap();
    return PublicKey::from_bytes(&encoded).unwrap();
}

#[cfg(test)]
mod test {
    use super::{gen_keypair, PrivateKey, PublicKey};

    #[test]
    fn test_key_serialization_correctness() {
        let keypair = gen_keypair();
        let privkey = keypair.private;
        let pubkey = keypair.public;

        let pubkey_bytes = pubkey.as_bytes();
        assert_eq!(PublicKey::from_bytes(&*pubkey_bytes), Some(pubkey));

        let privkey_bytes = privkey.as_bytes();
        assert_eq!(PrivateKey::from_bytes(&*privkey_bytes), Some(privkey));
    }

    #[test]
    fn test_key_serialization_robustness() {
        // 31 bytes should be too short for anything
        let bytes = [1u8; 31];
        assert_eq!(PublicKey::from_bytes(&bytes), None);
        assert_eq!(PrivateKey::from_bytes(&bytes), None);
    }
}
