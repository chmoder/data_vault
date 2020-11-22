use async_trait::async_trait;
use credit_card::CreditCard;
use deadpool_redis::PoolError as RedisPoolError;
use deadpool_postgres::PoolError as PostgresPoolError;
use std::error;


#[derive(Debug)]
pub enum PoolErrors {
    RedisPoolError,
    PostgresPoolError
}

impl From<RedisPoolError> for PoolErrors {
    fn from(_: RedisPoolError) -> Self {PoolErrors::RedisPoolError}
}

impl From<PostgresPoolError> for PoolErrors {
    fn from(_: PostgresPoolError) -> Self {PoolErrors::PostgresPoolError}
}

/// This is what a Data Vault can do
/// It's fundamental purpose is to store and retrieve
/// data in a secure encrypted manner
#[async_trait]
pub trait DataVault {
    fn new() -> Result<Self, Box<dyn error::Error>>
        where Self: std::marker::Sized;
    async fn store(&self, token: &String, string: &String) -> Result<(), PoolErrors>;
    async fn store_credit_card(&self, credit_card: &CreditCard) -> Result<String, PoolErrors>;
    async fn retrieve(&self, token: &String) -> Result<String, PoolErrors>;
    async fn retrieve_credit_card(&self, token: &String)  -> Result<CreditCard, PoolErrors>;
}