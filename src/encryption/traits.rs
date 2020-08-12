use block_modes::{Cbc};
use block_modes::block_padding::{Pkcs7};
use aes::Aes128;

pub trait Encryption {
    fn new() -> Self;
    fn encrypt(&self, bytes: &[u8]) -> Vec<u8>;
    fn encrypt_string(&self, text: &String) -> Vec<u8>;
    fn decrypt(&self, cipher_bytes: &[u8]) -> String;
    fn decrypt_vec(&self, cipher_vector: Vec<u8>) -> String;
}

pub trait Aes128CbcCipher {
    fn new_cipher(&self) -> Cbc<Aes128, Pkcs7>;
}