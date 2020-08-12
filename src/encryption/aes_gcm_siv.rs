use crate::config::EncryptionConfig;
use crate::encryption::traits::{Encryption};
use aes_gcm_siv::Aes256GcmSiv;
use aes_gcm_siv::aead::{Aead, NewAead, generic_array::GenericArray};


pub struct AesGcmSivEncryption {
    cipher: Aes256GcmSiv
}

/// High level encryption functionality for use
/// in DataVault Implementations
impl Encryption for AesGcmSivEncryption {
    /// use this class to add encryption to a data vault
    /// # Example
    /// ```rust
    /// use data_vault::encryption::traits::Aes128CbcCipher;
    /// use data_vault::encryption::traits::Aes128CbcEncryption;
    /// let enc = Aes128CbcEncryption::new();
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

    /// lowest level method that will encrypt data from this
    /// or higher level methods like `encrypt_string`
    /// # Example
    /// ```rust
    /// use data_vault::encryption::traits::Aes128CbcCipher;
    /// use data_vault::encryption::traits::Aes128CbcEncryption;
    ///
    /// let enc = Aes128CbcEncryption::new();
    /// let test_data = String::from("Hello world!");
    /// let encrypted_data = enc.encrypt(test_data.as_bytes());
    /// ```
    fn encrypt(&self, bytes: &[u8]) -> Vec<u8> {
        let nonce = GenericArray::from_slice(b"unique nonce");
        self.cipher.encrypt(nonce, bytes).unwrap()
    }

    /// encrypts `String` objects
    /// # Example
    /// ```rust
    /// use data_vault::encryption::traits::Aes128CbcCipher;
    /// use data_vault::encryption::traits::Aes128CbcEncryption;
    ///
    /// let enc = Aes128CbcEncryption::new();
    /// let test_data = String::from("Hello world!");
    /// let encrypted_data = enc.encrypt_string(&test_data);
    /// ```
    #[allow(dead_code)]
    fn encrypt_string(&self, text: &String) -> Vec<u8> {
        self.encrypt(text.as_bytes())
    }

    /// lowest level method to decrypt data
    /// # Example
    /// ```rust
    /// use data_vault::encryption::traits::Aes128CbcCipher;
    /// use data_vault::encryption::traits::Aes128CbcEncryption;
    ///
    /// let enc = Aes128CbcEncryption::new();
    /// let test_data = vec![27, 122, 76, 64, 49, 36, 174, 47, 181, 43, 237, 197, 52, 216, 47, 168];
    /// let encrypted_data = enc.decrypt_vec(test_data);
    /// ```
    fn decrypt(&self, cipher_bytes: &[u8]) -> String {
        let nonce = GenericArray::from_slice(
            self.nonce.as_bytes()
        );

        let decrypt_vec = self.cipher.decrypt(nonce, cipher_bytes).unwrap();
        String::from_utf8(decrypt_vec).unwrap_or_default()
    }

    /// decrypts a `Vec<u8>`
    /// # Example
    /// ```rust
    /// use data_vault::encryption::traits::Aes128CbcCipher;
    /// use data_vault::encryption::traits::Aes128CbcEncryption;
    ///
    /// let enc = Aes128CbcEncryption::new();
    /// let test_data = vec![27, 122, 76, 64, 49, 36, 174, 47, 181, 43, 237, 197, 52, 216, 47, 168];
    /// let encrypted_data = enc.decrypt(test_data.as_slice());
    /// ```
    #[allow(dead_code)]
    fn decrypt_vec(&self, cipher_vector: Vec<u8>) -> String {
        let cipher_bytes = cipher_vector.as_slice();
        self.decrypt(cipher_bytes)
    }
}