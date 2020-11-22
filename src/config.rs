use serde::Deserialize;
use dotenv::dotenv;

#[derive(Debug, Deserialize, Default)]
pub struct EncryptionConfig {
    pub key: String,
    pub iv: String,
    // cipher: Aes128Cbc,
}

#[derive(Debug, Deserialize, Default)]
pub struct DeadpoolRedisConfig {
    #[serde(default)]
    pub redis: deadpool_redis::Config,
}

#[derive(Debug, Deserialize, Default)]
pub struct DeadpoolPostgresConfig {
    #[serde(default)]
    pub postgres: deadpool_postgres::Config,
}

/// Populates a configuration from .env file or Environment Variables
/// for `encryption::Aes128CbcEncryption`.
/// Possible Values:
/// ENCRYPTED_DATA_VAULT_KEY=000102030405060708090a0b0c0d0e0f
/// ENCRYPTED_DATA_VAULT_IV=f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff
impl EncryptionConfig {
    pub fn from_env() -> Result<Self, ::config_crate::ConfigError> {
        dotenv().ok();
        let mut cfg = ::config_crate::Config::new();
        let environment = ::config_crate::Environment::new().separator("_").prefix("ENCRYPTED_DATA_VAULT");
        cfg.merge(environment).unwrap();
        cfg.try_into()
    }
}

/// Populates a configuration from .env file or Environment Variables
/// for `redis_data_vault::DeadpoolRedisConfig`.
/// Possible Values:
/// REDIS_URL=redis://:foobared@127.0.0.1/
/// REDIS_POOL_MAX_SIZE=16
impl DeadpoolRedisConfig {
    pub fn from_env() -> Result<Self, ::config_crate::ConfigError> {
        dotenv().ok();
        let mut cfg = ::config_crate::Config::new();
        let environment = ::config_crate::Environment::new().separator("_");
        cfg.merge(environment)?;
        cfg.try_into()
    }
}

/// Populates a configuration from .env file or Environment Variables
/// for `postgres_data_vault::DeadpoolPostgresConfig`.
/// Possible Values:
/// REDIS_URL=redis://:foobared@127.0.0.1/
/// REDIS_POOL_MAX_SIZE=16
impl DeadpoolPostgresConfig {
    pub fn from_env() -> Result<Self, ::config_crate::ConfigError> {
        dotenv().ok();
        let mut cfg = ::config_crate::Config::new();
        let environment = ::config_crate::Environment::new().separator(".");
        cfg.merge(environment)?;
        cfg.try_into()
    }
}

