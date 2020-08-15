use async_trait::async_trait;
use credit_card::CreditCard;
use deadpool_redis::PoolError;
use std::error;

/// This is what a Data Vault can do
/// It's fundamental purpose is to store and retrieve
/// data in a secure encrypted manner
#[async_trait]
pub trait DataVault {
    fn new() -> Result<Self, Box<dyn error::Error>>
        where Self: std::marker::Sized;
    async fn store(&self, token: &String, string: &String) -> Result<(), PoolError>;
    async fn store_credit_card(&self, credit_card: &CreditCard) -> Result<String, PoolError>;
    async fn retrieve(&self, token: &String) -> Result<String, PoolError>;
    async fn retrieve_credit_card(&self, token: &String)  -> Result<CreditCard, PoolError>;
}