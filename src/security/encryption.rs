use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand::Rng;
use crate::storage::blob_storage::EncryptionType;
use pqcrypto_kyber::kyber768;
use tfhe::boolean::prelude::*;

pub enum Encryptor {
    AES256(Aes256Gcm),
    PostQuantum(PostQuantumEncryptor),
    Homomorphic(HomomorphicEncryptor),
}

impl Encryptor {
    pub fn new(encryption_type: EncryptionType, key: &[u8]) -> Self {
        match encryption_type {
            EncryptionType::AES256 => {
                let cipher = Aes256Gcm::new(Key::from_slice(key));
                Encryptor::AES256(cipher)
            },
            EncryptionType::PostQuantum => {
                Encryptor::PostQuantum(PostQuantumEncryptor::new(key))
            },
            EncryptionType::Homomorphic => {
                Encryptor::Homomorphic(HomomorphicEncryptor::new(key))
            },
        }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        match self {
            Encryptor::AES256(cipher) => {
                let mut rng = rand::thread_rng();
                let nonce = Nonce::from_slice(&rng.gen::<[u8; 12]>());
                let ciphertext = cipher.encrypt(nonce, plaintext)?;
                let mut result = nonce.to_vec();
                result.extend_from_slice(&ciphertext);
                Ok(result)
            },
            Encryptor::PostQuantum(encryptor) => encryptor.encrypt(plaintext),
            Encryptor::Homomorphic(encryptor) => encryptor.encrypt(plaintext),
        }
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        match self {
            Encryptor::AES256(cipher) => {
                let nonce = Nonce::from_slice(&ciphertext[..12]);
                cipher.decrypt(nonce, &ciphertext[12..])
            },
            Encryptor::PostQuantum(encryptor) => encryptor.decrypt(ciphertext),
            Encryptor::Homomorphic(encryptor) => encryptor.decrypt(ciphertext),
        }
    }
}

pub struct PostQuantumEncryptor {
    public_key: kyber768::PublicKey,
    secret_key: kyber768::SecretKey,
}

impl PostQuantumEncryptor {
    pub fn new() -> Self {
        let (public_key, secret_key) = kyber768::keypair();
        Self { public_key, secret_key }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let (ciphertext, _) = kyber768::encapsulate(&self.public_key);
        ciphertext.to_vec()
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        let shared_secret = kyber768::decapsulate(
            kyber768::Ciphertext::from_bytes(ciphertext).unwrap(),
            &self.secret_key,
        );
        shared_secret.to_vec()
    }
}

pub struct HomomorphicEncryptor {
    client_key: ClientKey,
}

impl HomomorphicEncryptor {
    pub fn new() -> Self {
        let (client_key, _) = gen_keys();
        Self { client_key }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<bool> {
        plaintext.iter().flat_map(|&byte| {
            (0..8).map(move |i| {
                let bit = (byte >> i) & 1 != 0;
                self.client_key.encrypt(bit)
            })
        }).collect()
    }

    pub fn decrypt(&self, ciphertext: &[bool]) -> Vec<u8> {
        ciphertext.chunks(8)
            .map(|chunk| {
                chunk.iter().enumerate().fold(0u8, |acc, (i, &bit)| {
                    acc | (u8::from(self.client_key.decrypt(bit)) << i)
                })
            })
            .collect()
    }
}

// Implement PostQuantumEncryptor and HomomorphicEncryptor
