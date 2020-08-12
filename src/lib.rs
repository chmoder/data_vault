//! Data Vault is a modular, pragmatic, credit card vault for Rust.
//!
//! ## Example
//!
//! ```rust,ignore
//! use data_vault::DataVault;
//! use data_vault::RedisDataVault;
//! use credit_card::CreditCard;
//!
//! let vault = RedisDataVault::new();
//!
//! let cc = CreditCard {
//! number: "4111111111111111".to_string(),
//! cardholder_name: "Graydon Hoare".to_string(),
//! expiration_month: "01".to_string(),
//! expiration_year: "2023".to_string(),
//! brand: None,
//! security_code: None
//! };
//!
//! let token = vault.store_credit_card(&cc).await;
//! let credit_card = vault.retrieve_credit_card(&token.to_string()).await;
//! assert_eq!(credit_card.number, cc.number)
//! ```
//!
//! ## Features
//!
//! * Store [Credit Cards](https://github.com/chmoder/credit_card)
//! * Store `String`
//! * Automatic Encryption and Decryption
//! * Blake3 tokenization
//! * Redis Server, URL connection configuration
//! * Configurable from .env file or Environment Variables
//! * Runs on stable Rust

mod traits;
mod redis_data_vault;
mod config;
pub mod encryption;
pub mod tokenizer;

pub use traits::DataVault;
pub use redis_data_vault::RedisDataVault;


#[cfg(test)]
mod tests {
    use credit_card::CreditCard;
    use crate::traits::DataVault;
    use crate::redis_data_vault::RedisDataVault;
    use crate::encryption::traits::Encryption;
    use crate::encryption::AesGcmSivEncryption;

    #[tokio::test]
    async fn store_retrieve() {
        let vault = RedisDataVault::new();

        let cc = CreditCard {
            number: "4111111111111111".to_string(),
            cardholder_name: "Graydon Hoare".to_string(),
            expiration_month: "01".to_string(),
            expiration_year: "2023".to_string(),
            brand: None,
            security_code: None
        };

        let token = vault.store_credit_card(&cc).await;
        let credit_card = vault.retrieve_credit_card(&token.to_string()).await;
        assert_eq!(credit_card.number, cc.number)
    }

    #[test]
    fn test_encrypt_string() {
        let plaintext = "Hello world!".to_string();
        let x = hex::decode("3ca491c9cfc1097ecfaad15968daf4b22c4f032374cf40bd7398b6d4").unwrap();
        let expected_ciphertext = x.as_slice();

        let enc = AesGcmSivEncryption::new();
        let ciphertext = enc.encrypt_string(&plaintext);

        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn test_decrypt() {
        let plaintext = "Hello world!".to_string();
        let x = hex::decode("3ca491c9cfc1097ecfaad15968daf4b22c4f032374cf40bd7398b6d4").unwrap();
        let ciphertext = x.as_slice();

        let enc = AesGcmSivEncryption::new();
        let decrypted_ciphertext = enc.decrypt(&ciphertext);

        assert_eq!(decrypted_ciphertext, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let plaintext = "Hello world!".to_string();
        let enc = AesGcmSivEncryption::new();
        let ciphertext = enc.encrypt_string(&plaintext);
        let decrypted_ciphertext = enc.decrypt_vec(ciphertext);
        assert_eq!(decrypted_ciphertext, plaintext);
    }
}
