use async_trait::async_trait;
use crate::data_vault::{DataVault};
use credit_card::CreditCard;
use deadpool_redis::redis::AsyncCommands;
use crate::encryption::Encryption;
use crate::config::DeadpoolRedisConfig;


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
/// let data_vault = RedisDataVault::new();
/// ```
///
/// # Panics
/// Will panic when connection can not be made
pub struct RedisDataVault {
    pool: deadpool_redis::Pool,
    encryption: Encryption,
}

#[async_trait]
impl DataVault for RedisDataVault {
    /// Create new RedisDataVault backend
    fn new() -> Self {
        let cfg = DeadpoolRedisConfig::from_env().unwrap();

        RedisDataVault {
            pool: cfg.redis.create_pool().unwrap(),
            encryption: Encryption::new(),

        }
    }

    /// Store the credit card with the given token as the redis key
    /// * `token` - the token that maps to this credit card data (PAN)
    /// * `credit_card` - the cc object that you wish to store
    async fn store(&self, token: &String, string: &String) {
        let mut conn = self.pool.get().await.unwrap();
        let encrypted_json = self.encryption.encrypt(string.as_bytes());
        let _:() = conn.set(token, encrypted_json).await.unwrap_or_default();
    }


    /// Store the credit card with the given token as the redis key
    /// * `token` - the token that maps to this credit card data (PAN)
    /// * `credit_card` - the cc object that you wish to store
    async fn store_credit_card(&self, token: &String, credit_card: &CreditCard) {
        let credit_card_json = serde_json::to_string(&credit_card).unwrap();
        self.store(&token, &credit_card_json).await
    }

    /// Get the credit card from the data vault given token
    async fn retrieve(&self, token: &String) -> String {
        let mut conn = self.pool.get().await.unwrap();
        let encrypted_credit_card_json: Vec<u8> = conn.get(token).await.unwrap_or_default();
        self.encryption.decrypt(encrypted_credit_card_json.as_slice())
    }

    /// Get the credit card from the data vault given token
    async fn retrieve_credit_card(&self, token: &String) -> CreditCard {
        let credit_card_json = self.retrieve(token).await;
        serde_json::from_str(&credit_card_json).unwrap()
    }
}