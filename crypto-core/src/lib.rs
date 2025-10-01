use serde::{Deserialize, Serialize};
use rand::{RngCore, thread_rng};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeyPair {
    pub public_key: Vec<u8>,
    pub secret_key: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Signature {
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EncryptedMessage {
    pub ciphertext: Vec<u8>,
    pub encapsulated_key: Vec<u8>,
}

impl KeyPair {
    // Placeholder: generate random keys
    pub fn new_kem() -> Self {
        let mut public_key = vec![0; 32];
        let mut secret_key = vec![0; 32];
        thread_rng().fill_bytes(&mut public_key);
        thread_rng().fill_bytes(&mut secret_key);
        Self {
            public_key,
            secret_key,
        }
    }

    pub fn new_sig() -> Self {
        Self::new_kem()
    }
}

// Placeholder encryption: simple XOR with public key as key
pub fn encrypt(plaintext: &[u8], public_key: &[u8]) -> EncryptedMessage {
    let key = public_key.iter().cycle().take(plaintext.len()).collect::<Vec<_>>();
    let ciphertext = plaintext.iter().zip(key).map(|(p, k)| p ^ k).collect::<Vec<_>>();
    let encapsulated_key = public_key.to_vec();
    EncryptedMessage {
        ciphertext,
        encapsulated_key,
    }
}

pub fn decrypt(encrypted: &EncryptedMessage, secret_key: &[u8]) -> Vec<u8> {
    encrypt(&encrypted.ciphertext, secret_key).ciphertext
}

// Placeholder signature: append "sig" to message
pub fn sign(message: &[u8], _secret_key: &[u8]) -> Signature {
    let mut data = b"sig".to_vec();
    data.extend(message);
    Signature { data }
}

pub fn verify(message: &[u8], signature: &[u8], _public_key: &[u8]) -> bool {
    signature.starts_with(b"sig")
}

// WASM bindings
#[wasm_bindgen]
pub fn generate_key_pair() -> JsValue {
    let key_pair = KeyPair::new_kem();
    serde_wasm_bindgen::to_value(&key_pair).unwrap()
}

#[wasm_bindgen]
pub fn encrypt_message(plaintext: &[u8], public_key: &[u8]) -> JsValue {
    let encrypted = encrypt(plaintext, public_key);
    serde_wasm_bindgen::to_value(&encrypted).unwrap()
}

#[wasm_bindgen]
pub fn decrypt_message(ciphertext: &[u8], encapsulated_key: &[u8], secret_key: &[u8]) -> Vec<u8> {
    let encrypted = EncryptedMessage {
        ciphertext: ciphertext.to_vec(),
        encapsulated_key: encapsulated_key.to_vec(),
    };
    decrypt(&encrypted, secret_key)
}

#[wasm_bindgen]
pub fn sign_message(message: &[u8], secret_key: &[u8]) -> JsValue {
    let signature = sign(message, secret_key);
    serde_wasm_bindgen::to_value(&signature).unwrap()
}

#[wasm_bindgen]
pub fn verify_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    verify(message, signature, public_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kem_keygen() {
        let kp = KeyPair::new_kem();
        assert!(!kp.public_key.is_empty());
        assert!(!kp.secret_key.is_empty());
    }

    #[test]
    fn test_kem_encrypt_decrypt() {
        let alice_keys = KeyPair::new_kem();

        let message = b"Hello, quantum world!";
        let encrypted = encrypt(message, &alice_keys.public_key);
        let decrypted = decrypt(&encrypted, &alice_keys.secret_key);

        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_sign_verify() {
        let keys = KeyPair::new_sig();
        let message = b"Sign this message";

        let signature = sign(message, &keys.secret_key);
        let verified = verify(message, &signature.data, &keys.public_key);

        assert!(verified);
    }
}
