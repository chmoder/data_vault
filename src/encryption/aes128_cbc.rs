use crate::config::EncryptionConfig;
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

// create an alias for convenience
type Aes128Cbc = Cbc<Aes128, Pkcs7>;


pub struct Aes128CbcEncryption {
    key: Vec<u8>,
    iv: Vec<u8>,
    // cipher: Aes128Cbc
}

/// High level encryption functionality for use
/// in DataVault Implementations
impl Aes128CbcEncryption {
    /// use this class to add encryption to a data vault
    /// # Example
    /// ```rust
    /// use data_vault::encryption::Aes128CbcEncryption;
    /// let enc = Aes128CbcEncryption::new();
    /// ```
    pub fn new() -> Self {
        let cfg = EncryptionConfig::from_env().unwrap();

        let key = hex::decode(cfg.key).unwrap();
        let iv = hex::decode(cfg.iv).unwrap();
        // let cipher = Aes128Cbc::new_var(
        //     key.clone().as_slice(),
        //     iv.clone().as_slice()
        // ).unwrap();

        Self {
            key,
            iv,
            // cipher,
        }
    }

    /// Creates a new instance of the cipher
    /// This is a temporary work around to issues
    /// borrowing from self.cipher Aes128Cbc does not implement Copy, Clone
    /// Like:
    /// move occurs because value has type `block_modes::cbc::Cbc<aes_soft::impls::Aes128,
    /// block_padding::Pkcs7>`, which does not implement the `Copy` trait
    fn new_cipher(&self) -> Cbc<Aes128, Pkcs7> {
        Aes128Cbc::new_var(self.key.as_slice(), self.iv.as_slice()).unwrap()
    }

    /// lowest level method that will encrypt data from this
    /// or higher level methods like `encrypt_string`
    /// # Example
    /// ```rust
    /// use data_vault::encryption::Aes128CbcEncryption;
    ///
    /// let enc = Aes128CbcEncryption::new();
    /// let test_data = String::from("Hello world!");
    /// let encrypted_data = enc.encrypt(test_data.as_bytes());
    /// ```
    pub fn encrypt(&self, bytes: &[u8]) -> Vec<u8> {
        self.new_cipher().encrypt_vec(bytes)
    }

    /// encrypts `String` objects
    /// # Example
    /// ```rust
    /// use data_vault::encryption::Aes128CbcEncryption;
    ///
    /// let enc = Aes128CbcEncryption::new();
    /// let test_data = String::from("Hello world!");
    /// let encrypted_data = enc.encrypt_string(&test_data);
    /// ```
    #[allow(dead_code)]
    pub fn encrypt_string(&self, text: &String) -> Vec<u8> {
        self.encrypt(text.as_bytes())
    }

    /// lowest level method to decrypt data
    /// # Example
    /// ```rust
    /// use data_vault::encryption::Aes128CbcEncryption;
    ///
    /// let enc = Aes128CbcEncryption::new();
    /// let test_data = vec![27, 122, 76, 64, 49, 36, 174, 47, 181, 43, 237, 197, 52, 216, 47, 168];
    /// let encrypted_data = enc.decrypt_vec(test_data);
    /// ```
    pub fn decrypt(&self, cipher_bytes: &[u8]) -> String {
        let decrypt_vec = self.new_cipher().decrypt_vec(cipher_bytes).unwrap();
        String::from_utf8(decrypt_vec).unwrap_or_default()
    }

    /// decrypts a `Vec<u8>`
    /// # Example
    /// ```rust
    /// use data_vault::encryption::Aes128CbcEncryption;
    ///
    /// let enc = Aes128CbcEncryption::new();
    /// let test_data = vec![27, 122, 76, 64, 49, 36, 174, 47, 181, 43, 237, 197, 52, 216, 47, 168];
    /// let encrypted_data = enc.decrypt(test_data.as_slice());
    /// ```
    #[allow(dead_code)]
    pub fn decrypt_vec(&self, cipher_vector: Vec<u8>) -> String {
        let cipher_bytes = cipher_vector.as_slice();
        self.decrypt(cipher_bytes)
    }
}
