use log::{info};
use tokio;
use credit_card::CreditCard;
use data_vault::{RedisDataVault, DataVault};
use std::vec;
use std::time::{Instant};
use std::sync::Arc;
use data_vault::encryption::AesGcmSivEncryption;
use data_vault::tokenizer::Blake3Tokenizer;

#[tokio::main(core_threads = 4)]
async fn main() {
    env_logger::init();

    let mut token_futures = vec::Vec::new();
    let redis_vault = RedisDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();
    let vault = Arc::new(redis_vault);
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
        let redis_result = token_result.unwrap();
        let token = redis_result.unwrap();
        let _credit_card = vault.retrieve_credit_card(&token).await.unwrap();
    }

    let end_time = Instant::now();
    info!("retrieved {} credit cards in {:?}", to_store, end_time.duration_since(stored_time));
    info!("tokenized, stored, and retrieved {} credit cards in {:?}", to_store, end_time.duration_since(start_time));
}
