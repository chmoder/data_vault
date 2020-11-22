//! Data Vault is a modular, pragmatic, credit card vault for Rust.
//!
//! ## Example
//!
//! ```toml
//! ## Cargo.toml
//! [dependencies]
//! data_vault = "^0.2"
//! ```
//!
//! ```dotenv
//! ## .env OR environment variables
//! REDIS_URL=redis://:foobared@127.0.0.1/
//! ## REDIS_POOL_MAX_SIZE=16
//! ENCRYPTED_DATA_VAULT_KEY=000102030405060708090a0b0c0d0e0f
//! ENCRYPTED_DATA_VAULT_IV=f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff
//! ```
//!
//! ```rust
//! /// example.rs
//!
//! // traits
//! use data_vault::DataVault;
//! use data_vault::encryption::traits::Encryption;
//!
//! // data vault
//! use data_vault::RedisDataVault;
//! use data_vault::PostgresDataVault;
//! // Interchangeable encryption
//! use data_vault::encryption::AesGcmSivEncryption;
//! // Interchangeable tokenizer
//! use data_vault::tokenizer::Blake3Tokenizer;
//!
//! // credit card type
//! use credit_card::CreditCard;
//!
//! use tokio;
//!
//! #[tokio::main(core_threads = 4)]
//! async fn main() {
//!     let vault = RedisDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();
//!
//!     let cc = CreditCard {
//!         number: "4111111111111111".to_string(),
//!         cardholder_name: "Graydon Hoare".to_string(),
//!         expiration_month: "01".to_string(),
//!         expiration_year: "2023".to_string(),
//!         brand: None,
//!         security_code: None
//!     };
//!
//!     let token = vault.store_credit_card(&cc).await.unwrap();
//!     let credit_card = vault.retrieve_credit_card(&token.to_string()).await.unwrap();
//!     assert_eq!(credit_card.number, cc.number)
//! }
//! ```
//!
//! # Current Features
//! - Store [Credit Cards](https://github.com/chmoder/credit_card)
//! - Store `String`
//! - Automatic Encryption and Decryption
//! - Blake3 tokenization
//! - Redis Server, URL connection configuration
//! - Configurable from .env file or Environment Variables
//! - Interchangeable Encryption
//! - Interchangeable Tokenization hasher
//!
//! # Future Features
//! - Postgres Database
//!
//! # Performance
//! This [example](https://github.com/chmoder/data_vault/blob/master/examples/benchmark.rs) output the following performance stats with an AMD Ryzen 9 3900X.
//! Showing the possibility of tokenizing **~100,000** credit cards per second.
//!
//! tokenized and stored 100000 credit cards in 1.058474365s
//! retrieved 100000 credit cards in 5.353857633s
//! tokenized, stored, and retrieved 100000 credit cards in 6.412331998s
//!

mod traits;
mod redis_data_vault;
mod postgres_data_vault;
mod config;
pub mod utils;
pub mod encryption;
pub mod tokenizer;

pub use traits::DataVault;
pub use redis_data_vault::RedisDataVault;
pub use postgres_data_vault::PostgresDataVault;


#[cfg(test)]
mod tests {
    use credit_card::CreditCard;
    use crate::traits::DataVault;
    use crate::redis_data_vault::RedisDataVault;
    use crate::encryption::traits::Encryption;
    use crate::encryption::AesGcmSivEncryption;
    use crate::tokenizer::Blake3Tokenizer;
    use crate::PostgresDataVault;

    #[tokio::test(flavor = "multi_thread")]
    async fn store_retrieve_redis() {
        let vault = RedisDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();

        let cc = CreditCard {
            number: "4111111111111111".to_string(),
            cardholder_name: "Graydon Hoare".to_string(),
            expiration_month: "01".to_string(),
            expiration_year: "2023".to_string(),
            brand: None,
            security_code: None
        };

        let token = vault.store_credit_card(&cc).await.unwrap();
        let credit_card = vault.retrieve_credit_card(&token.to_string()).await.unwrap();
        assert_eq!(credit_card.number, cc.number)
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn store_retrieve_postgres() {
        let vault = PostgresDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();

        let cc = CreditCard {
            number: "4111111111111111".to_string(),
            cardholder_name: "Graydon Hoare".to_string(),
            expiration_month: "01".to_string(),
            expiration_year: "2023".to_string(),
            brand: None,
            security_code: None
        };

        let token = vault.store_credit_card(&cc).await.unwrap();
        let credit_card = vault.retrieve_credit_card(&token.to_string()).await.unwrap();
        assert_eq!(credit_card.number, cc.number)
    }

    #[test]
    fn test_encrypt_string() {
        let plaintext = "Hello world!".to_string();
        // let x = hex::decode("feb261e2d1ead79e0adcdf95d743b97d2a875603d10476f1fef2df5f").unwrap();
        // let expected_ciphertext = x.as_slice();

        let enc = AesGcmSivEncryption::new();
        let data = enc.encrypt_string(&plaintext);
        let decrypted_data = enc.decrypt_vec(data);
        // let (nonce, ciphertext) = data.split_at(12);

        assert_eq!(plaintext, decrypted_data);
    }

    #[test]
    fn test_decrypt() {
        let plaintext = "Hello world!".to_string();
        let nonce = b"unique nonce";
        let x = hex::decode("3ca491c9cfc1097ecfaad15968daf4b22c4f032374cf40bd7398b6d4").unwrap();
        let ciphertext = [nonce, x.as_slice()].concat();

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
