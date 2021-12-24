use std::iter;
use crate::errors::DaMieError;
use chacha20poly1305::{XChaCha20Poly1305, Key, XNonce};
use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::aead::generic_array::GenericArray;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn encrypt_chacha(cleartext: Vec<u8>, key: &str) -> Vec<u8>{
    let key = Key::from_slice(key.as_bytes());
    let aead = XChaCha20Poly1305::new(&key);
    let nonce = XNonce::from_slice(b"extra long unique nonce!");
    let ciphertext: Vec<u8> = aead
        .encrypt(nonce, cleartext.as_ref())
        .expect("encryption failure!");
    ciphertext
}

pub fn decrypt_chacha(ciphertext: Vec<u8>, key: &str) -> Vec<u8> {
    let key = Key::from_slice(key.as_bytes());
    let aead = XChaCha20Poly1305::new(&key);
    let nonce = XNonce::from_slice(b"extra long unique nonce!");
    let plaintext: Vec<u8> = aead
        .decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!");
    plaintext
}