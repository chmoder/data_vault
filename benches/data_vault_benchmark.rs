use tokio;
use criterion::{criterion_group, criterion_main, Criterion};
use credit_card::CreditCard;
use data_vault::{RedisDataVault, DataVault};
use core::time::Duration;
use data_vault::encryption::AesGcmSivEncryption;
use data_vault::tokenizer::Blake3Tokenizer;

#[tokio::main]
async fn store_retrieve_credit_card() {
    let vault = RedisDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();

    let cc = CreditCard {
        number: "4111111111111111".to_string(),
        cardholder_name: "Graydon Hoare".to_string(),
        expiration_month: "01".to_string(),
        expiration_year: "2023".to_string(),
        brand: None,
        security_code: None
    };

    let token = vault.store_credit_card(&cc).await.unwrap();
    let credit_card = vault.retrieve_credit_card(&token.to_string()).await.unwrap();
    assert_eq!(credit_card.number, cc.number)
}

#[tokio::main]
async fn store_credit_card() {
    let vault = RedisDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();

    let cc = CreditCard {
        number: "4111111111111111".to_string(),
        cardholder_name: "Graydon Hoare".to_string(),
        expiration_month: "01".to_string(),
        expiration_year: "2023".to_string(),
        brand: None,
        security_code: None
    };

    let token = vault.store_credit_card(&cc).await.unwrap();
    assert_eq!(token.len(), 64)
}

#[tokio::main]
async fn retrieve_credit_card() {
    let vault = RedisDataVault::<AesGcmSivEncryption,Blake3Tokenizer>::new().unwrap();

    let token = "token";
    let mut credit_card = vault.retrieve_credit_card(&token.to_string()).await.unwrap();

    if credit_card.number.len() == 0 {
        let cc = CreditCard {
            number: "4111111111111111".to_string(),
            cardholder_name: "Graydon Hoare".to_string(),
            expiration_month: "01".to_string(),
            expiration_year: "2023".to_string(),
            brand: None,
            security_code: None
        };

        let credit_card_json = serde_json::to_string(&cc).unwrap();
        let _ = vault.store(&token.to_string(), &credit_card_json).await.unwrap();
        credit_card = vault.retrieve_credit_card(&token.to_string()).await.unwrap();
    }

    assert_eq!(credit_card.number, "4111111111111111".to_string())
}

fn criterion_store_credit_card(c: &mut Criterion) {
    c.bench_function("store", |b| b.iter(|| store_credit_card()));
}

fn criterion_retrieve_credit_card(c: &mut Criterion) {
    c.bench_function("retrieve", |b| b.iter(|| retrieve_credit_card()));
}

fn criterion_store_retrieve_credit_card(c: &mut Criterion) {
    c.bench_function("store_retrieve", |b| b.iter(|| store_retrieve_credit_card()));
}


criterion_group!(benches, criterion_store_credit_card, criterion_retrieve_credit_card, criterion_store_retrieve_credit_card );
criterion_main!(benches);