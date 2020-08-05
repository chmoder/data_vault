use async_trait::async_trait;
use credit_card::CreditCard;
use deadpool_redis::redis::AsyncCommands;
use crate::data_vault::{DataVault};
use crate::config::DeadpoolRedisConfig;
use crate::encryption::Aes128CbcEncryption;
use crate::tokenizer::{Tokenizer, Blake3Tokenizer};

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
/// use data_vault::{RedisDataVault, DataVault};
/// let data_vault = RedisDataVault::new();
/// ```
///
/// # Panics
/// Will panic when connection can not be made
pub struct RedisDataVault {
    pool: deadpool_redis::Pool,
    encryption: Aes128CbcEncryption,
    tokenizer: Blake3Tokenizer
}

#[async_trait]
impl DataVault for RedisDataVault {
    /// Create new RedisDataVault backend
    fn new() -> Self {
        let cfg = DeadpoolRedisConfig::from_env().unwrap();

        RedisDataVault {
            pool: cfg.redis.create_pool().unwrap(),
            encryption: Aes128CbcEncryption::new(),
            tokenizer: Blake3Tokenizer::new()
        }
    }

    /// Store the credit card with the given token as the redis key
    /// * `token` - the token that maps to this credit card data (PAN)
    /// * `credit_card` - the cc object that you wish to store
    async fn store(&self, credit_card: &CreditCard) -> String {
        let token = self.tokenizer.generate(&credit_card);

        let mut conn = self.pool.get().await.unwrap();
        let credit_card_json = serde_json::to_string(&credit_card).unwrap();
        let encrypted_json = self.encryption.encrypt(credit_card_json.as_bytes());
        let _:() = conn.set(&token, encrypted_json).await.unwrap_or_default();
        token
    }

    /// Get the credit card from the data vault given token
    async fn retrieve(&self, token: &String) -> CreditCard {
        let mut conn = self.pool.get().await.unwrap();
        let encrypted_credit_card_json: Vec<u8> = conn.get(token).await.unwrap_or_default();
        let credit_card_json = self.encryption.decrypt(encrypted_credit_card_json.as_slice());
        serde_json::from_str(&credit_card_json).unwrap_or_default()
    }
}