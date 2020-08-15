use crate::config::EncryptionConfig;
use crate::encryption::traits::{Encryption};
use aes_gcm_siv::Aes256GcmSiv;
use aes_gcm_siv::aead::{Aead, NewAead, generic_array::GenericArray};
use crate::utils::Salt;

const NONCE_SIZE: u8 = 12;

pub struct AesGcmSivEncryption {
    cipher: Aes256GcmSiv
}

/// High level encryption functionality for use
/// in DataVault Implementations
impl Encryption for AesGcmSivEncryption {
    /// use this struct to add encryption to a data vault
    /// # Example
    /// ```rust
    /// use data_vault::encryption::traits::Encryption;
    /// use data_vault::encryption::AesGcmSivEncryption;
    /// let enc = AesGcmSivEncryption::new();
    /// ```
    fn new() -> Self {
        let cfg = EncryptionConfig::from_env().unwrap();
        let key = GenericArray::from_slice(
            cfg.key.as_bytes()
        );

        let cipher = Aes256GcmSiv::new(key);

        Self {
            cipher
        }
    }

    /// The lowest level method for encrypting data.
    /// Encrypts `bytes` and prepends a 12 byte nonce
    /// to the encrypted data.
    ///
    /// # Arguments
    ///
    /// `bytes` - Aata to encrypt
    ///
    /// # Example
    /// ```rust
    /// use data_vault::encryption::traits::Encryption;
    /// use data_vault::encryption::AesGcmSivEncryption;
    ///
    /// let enc = AesGcmSivEncryption::new();
    /// let test_data = String::from("Hello world!");
    /// let encrypted_data = enc.encrypt(test_data.as_bytes());
    /// ```
    fn encrypt(&self, bytes: &[u8]) -> Vec<u8> {
        let nonce_string = Salt::generate(NONCE_SIZE as usize);
        let nonce = GenericArray::from_slice(nonce_string.as_bytes());
        let cipher_text = self.cipher.encrypt(nonce, bytes).unwrap();
        [nonce_string.as_bytes().to_vec(), cipher_text].concat()
    }

    /// Encrypts `String` objects.
    ///
    /// # Arguments
    ///
    /// `text`: - text data to encrypt
    ///
    /// # Example
    /// ```rust
    /// use data_vault::encryption::traits::Encryption;
    /// use data_vault::encryption::AesGcmSivEncryption;
    ///
    /// let enc = AesGcmSivEncryption::new();
    /// let test_data = String::from("Hello world!");
    /// let encrypted_data = enc.encrypt_string(&test_data);
    /// ```
    #[allow(dead_code)]
    fn encrypt_string(&self, text: &String) -> Vec<u8> {
        self.encrypt(text.as_bytes())
    }

    /// The lowest level method to decrypt data
    ///
    /// # Arguments
    ///
    /// `bytes` - byte data to decrypt.  The first 12 bytes must be a Nonce value
    ///
    /// # Example
    /// ```rust
    /// use data_vault::encryption::traits::Encryption;
    /// use data_vault::encryption::AesGcmSivEncryption;
    ///
    /// let enc = AesGcmSivEncryption::new();
    /// let nonce = "unique nonce".as_bytes();
    /// let test_data = vec![85, 117, 109, 67, 71, 109, 74, 66, 55, 100, 119, 70, 208, 88, 64, 198, 33, 160, 61, 101, 8, 179, 140, 90, 139, 124, 195, 110, 120, 216, 244, 143, 128, 208, 90, 61, 127, 37, 35, 235];
    /// let encrypted_data = enc.decrypt_vec(test_data);
    /// ```
    fn decrypt(&self, bytes: &[u8]) -> String {
        let (nonce_bytes, cipher_bytes) = bytes.split_at(12);
        let nonce = GenericArray::from_slice(nonce_bytes);
        let decrypt_vec = self.cipher.decrypt(nonce, cipher_bytes).unwrap();
        String::from_utf8(decrypt_vec).unwrap_or_default()
    }

    /// decrypts a `Vec<u8>`
    ///
    /// # Arguments
    ///
    /// `cipher_vector` - Vectorized data to decrypt.  The first 12 bytes must be a Nonce value.
    ///
    /// # Example
    /// ```rust
    /// use data_vault::encryption::traits::Encryption;
    /// use data_vault::encryption::AesGcmSivEncryption;
    ///
    /// let enc = AesGcmSivEncryption::new();
    /// let test_data = vec![85, 117, 109, 67, 71, 109, 74, 66, 55, 100, 119, 70, 208, 88, 64, 198, 33, 160, 61, 101, 8, 179, 140, 90, 139, 124, 195, 110, 120, 216, 244, 143, 128, 208, 90, 61, 127, 37, 35, 235];
    /// let encrypted_data = enc.decrypt(test_data.as_slice());
    /// ```
    #[allow(dead_code)]
    fn decrypt_vec(&self, cipher_vector: Vec<u8>) -> String {
        let cipher_bytes = cipher_vector.as_slice();
        self.decrypt(cipher_bytes)
    }
}