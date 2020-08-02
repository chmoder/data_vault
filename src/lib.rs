mod config;
mod encryption;
mod data_vault;
mod redis_data_vault;

pub use data_vault::DataVault;
pub use redis_data_vault::RedisDataVault;


#[cfg(test)]
mod tests {
    use credit_card::CreditCard;
    use crate::{RedisDataVault, DataVault};
    use crate::encryption::Encryption;

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

        vault.store_credit_card(&"token".to_string(), &cc).await;
        let credit_card = vault.retrieve_credit_card(&"token".to_string()).await;
        assert_eq!(credit_card.number, cc.number)
    }

    #[test]
    fn test_encrypt_string() {
        let plaintext = "Hello world!".to_string();
        let x = hex::decode("1b7a4c403124ae2fb52bedc534d82fa8").unwrap();
        let expected_ciphertext = x.as_slice();

        let enc = Encryption::new();
        let ciphertext = enc.encrypt_string(plaintext.clone());

        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn test_decrypt() {
        let plaintext = "Hello world!".to_string();
        let x = hex::decode("1b7a4c403124ae2fb52bedc534d82fa8").unwrap();
        let ciphertext = x.as_slice();

        let enc = Encryption::new();
        let decrypted_ciphertext = enc.decrypt(&ciphertext);

        assert_eq!(decrypted_ciphertext, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let plaintext = "Hello world!".to_string();
        let enc = Encryption::new();
        let ciphertext = enc.encrypt_string(plaintext.clone());
        let decrypted_ciphertext = enc.decrypt_vec(ciphertext);
        assert_eq!(decrypted_ciphertext, plaintext);
    }
}
