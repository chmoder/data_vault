use async_trait::async_trait;
use credit_card::CreditCard;
use deadpool_redis::redis::{AsyncCommands};
use crate::traits::{DataVault, PoolErrors};
use crate::config::DeadpoolRedisConfig;
use crate::encryption::traits::Encryption;
use crate::tokenizer::{Tokenizer};
use std::error;

/// Use redis as a data vault back end
///
/// This implementation uses deadpool_redis
///
/// Connection setup is available as environment
/// variables or a .env file with the following
/// options:
/// REDIS_URL=redis://127.0.0.1/
/// REDIS_POOL_MAX_SIZE=16
///
/// # Examples
/// ```rust
/// use data_vault::{DataVault, RedisDataVault};
/// use data_vault::encryption::AesGcmSivEncryption;
/// use data_vault::tokenizer::Blake3Tokenizer;
/// let data_vault = RedisDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();
/// ```
pub struct RedisDataVault<E, T> {
    pool: deadpool_redis::Pool,
    encryption: E,
    tokenizer: T,
}

#[async_trait]
impl<E, T> DataVault for RedisDataVault<E, T>
    where
        E: Encryption + std::marker::Sync + std::marker::Send,
        T: Tokenizer + std::marker::Sync + std::marker::Send,
{
    /// Create new RedisDataVault backend
    /// # examples
    /// ```rust
    /// use data_vault::{DataVault, RedisDataVault};
    /// use data_vault::encryption::AesGcmSivEncryption;
    /// use data_vault::tokenizer::Blake3Tokenizer;
    /// let data_vault = RedisDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();
    /// ```
    fn new() -> Result<Self, Box<dyn error::Error>> {
        let cfg = DeadpoolRedisConfig::from_env()?;

        let pool = cfg.redis.create_pool()?;

        let redis_data_vault = RedisDataVault {
            pool,
            encryption: E::new(),
            tokenizer: T::new()
        };

        Ok(redis_data_vault)
    }

    /// Encrypt and Store a string with the given token as the redis key
    /// Arguments:
    ///     * `CreditCard` - the cc object that you wish to store
    /// return:
    ///     the token as String
    /// # example
    /// ```rust
    /// use data_vault::{DataVault, RedisDataVault};
    /// use data_vault::encryption::AesGcmSivEncryption;
    /// use data_vault::tokenizer::Blake3Tokenizer;
    ///
    /// let token = String::from("abc123");
    /// let credit_card_string = String::from("{number: 123}");
    /// let data_vault = RedisDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();
    /// data_vault.store(&token, &credit_card_string);
    /// ```
    async fn store(&self, token: &String, string: &String) -> Result<(), PoolErrors> {
        let mut conn = self.pool.get().await?;
        let encrypted_json = self.encryption.encrypt(string.as_bytes());
        let _:() = conn.set(token, encrypted_json).await.unwrap();
        Ok(())
    }

    /// Store the credit card in the data vault
    /// Arguments:
    ///     * `CreditCard` - the cc object that you wish to store
    /// return:
    ///     A new token as String
    /// # example
    /// ```rust
    /// use data_vault::{DataVault, RedisDataVault};
    /// use data_vault::encryption::AesGcmSivEncryption;
    /// use data_vault::tokenizer::Blake3Tokenizer;
    /// use credit_card::CreditCard;
    ///
    /// let cc = CreditCard {
    ///    number: "4111111111111111".to_string(),
    ///    cardholder_name: "Graydon Hoare".to_string(),
    ///    expiration_month: "01".to_string(),
    ///    expiration_year: "2023".to_string(),
    ///    brand: None,
    ///    security_code: None
    /// };
    ///
    /// let data_vault = RedisDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();
    /// let token = data_vault.store_credit_card(&cc);
    /// ```
    async fn store_credit_card(&self, credit_card: &CreditCard) -> Result<String, PoolErrors> {
        let token = self.tokenizer.generate(&credit_card);
        let credit_card_json = serde_json::to_string(&credit_card).unwrap();
        let _:() = self.store(&token, &credit_card_json).await?;
        Ok(token)
    }

    /// Get decrypted arbitrary data from the vault by token
    /// Arguments:
    ///     * `token`: the string form of the ID of the data
    /// returns:
    ///     * the decrypted string of data
    /// # example
    /// ```rust,ignore
    /// use data_vault::DataVault;
    /// use data_vault::RedisDataVault;
    /// use data_vault::encryption::AesGcmSivEncryption;
    /// use data_vault::tokenizer::Blake3Tokenizer;
    ///
    /// let token = String::from("abc123");
    /// let cc_string = String::from("{number: 123}");
    /// let data_vault = RedisDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();
    /// data_vault.store(&token, &cc_string).await;
    /// let credit_card_string = data_vault.retrieve(&token)
    /// ```
    async fn retrieve(&self, token: &String) -> Result<String, PoolErrors> {
        let mut conn = self.pool.get().await?;
        let encrypted_credit_card_json: Vec<u8> = conn.get(token).await.unwrap();
        Ok(self.encryption.decrypt(encrypted_credit_card_json.as_slice()))
    }

    /// Get the credit card from the data vault given a token
    /// Arguments:
    ///     * `token`: the string form of the ID of the data
    /// returns:
    ///     * `CreditCard` object
    /// # Example
    /// ```rust,ignore
    /// use data_vault::DataVault;
    /// use data_vault::RedisDataVault;
    /// use data_vault::encryption::AesGcmSivEncryption;
    /// use data_vault::tokenizer::Blake3Tokenizer;
    /// use credit_card::CreditCard;
    ///
    /// let cc = CreditCard {
    ///    number: "4111111111111111".to_string(),
    ///    cardholder_name: "Graydon Hoare".to_string(),
    ///    expiration_month: "01".to_string(),
    ///    expiration_year: "2023".to_string(),
    ///    brand: None,
    ///    security_code: None
    /// };
    ///
    /// let data_vault = RedisDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();
    /// let token = data_vault.store_credit_card(&cc).await;
    /// let credit_card = data_vault.retrieve_credit_card(&token).await;
    /// ```
    async fn retrieve_credit_card(&self, token: &String) -> Result<CreditCard, PoolErrors> {
        let credit_card_json = self.retrieve(token).await?;
        Ok(serde_json::from_str(&credit_card_json).unwrap_or_default())
    }
}