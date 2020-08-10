use log::{info};
use tokio;
use credit_card::CreditCard;
use data_vault::{RedisDataVault, DataVault};
use std::vec;
use std::time::{Instant};
use std::sync::Arc;

#[tokio::main(core_threads = 4)]
async fn main() {
    env_logger::init();

    let mut token_futures = vec::Vec::new();
    let vault = Arc::new(RedisDataVault::new());
    let cc = Arc::new(CreditCard {
        number: "4111111111111111".to_string(),
        cardholder_name: "Graydon Hoare".to_string(),
        expiration_month: "01".to_string(),
        expiration_year: "2023".to_string(),
        brand: None,
        security_code: None
    });

    let to_store:i32 = 100000;
    info!("start");
    let start_time = Instant::now();

    for _ in 0..to_store {
        let vault = vault.clone();
        let cc = Arc::clone(&cc);
        token_futures.push(
            tokio::task::spawn(async move {
                vault.store_credit_card(&cc).await
            })
        );
    }
    let results = futures::future::join_all(token_futures).await;

    let stored_time = Instant::now();
    info!("tokenized and stored {} credit cards in {:?}", to_store, stored_time.duration_since(start_time));

    for token_result in results {
        let token = token_result.unwrap();
        vault.retrieve_credit_card(&token).await;
    }

    let end_time = Instant::now();
    info!("retrieved {} credit cards in {:?}", to_store, end_time.duration_since(stored_time));
    info!("tokenized, stored, and retrieved {} credit cards in {:?}", to_store, end_time.duration_since(start_time));
}
