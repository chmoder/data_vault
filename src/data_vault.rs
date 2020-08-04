use async_trait::async_trait;
use credit_card::CreditCard;

/// This is what a Data Vault can do
/// It's fundamental purpose is to store and retrieve
/// data in a secure encrypted manner
#[async_trait]
pub trait DataVault {
    fn new() -> Self;
    async fn store(&self, credit_card: &CreditCard) -> String;
    async fn retrieve(&self, token: &String) -> CreditCard;
}