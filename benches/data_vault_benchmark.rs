use tokio;
use criterion::{criterion_group, criterion_main, Criterion};
use credit_card::CreditCard;
use data_vault::{RedisDataVault, DataVault, PostgresDataVault};
use data_vault::encryption::AesGcmSivEncryption;
use data_vault::tokenizer::Blake3Tokenizer;

#[tokio::main]
async fn store_retrieve_credit_card_redis() {
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
async fn store_credit_card_redis() {
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
async fn retrieve_credit_card_redis() {
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

#[tokio::main]
async fn store_retrieve_credit_card_postgres() {
    let vault = PostgresDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();

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
async fn store_credit_card_postgres() {
    let vault = PostgresDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();

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
async fn retrieve_credit_card_postgres() {
    let vault = PostgresDataVault::<AesGcmSivEncryption,Blake3Tokenizer>::new().unwrap();

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

// redis
fn criterion_store_credit_card_redis(c: &mut Criterion) {
    c.bench_function("store_redis", |b| b.iter(|| store_credit_card_redis()));
}

fn criterion_retrieve_credit_card_redis(c: &mut Criterion) {
    c.bench_function("retrieve_redis", |b| b.iter(|| retrieve_credit_card_redis()));
}

fn criterion_store_retrieve_credit_card_redis(c: &mut Criterion) {
    c.bench_function("store_retrieve_redis", |b| b.iter(|| store_retrieve_credit_card_redis()));
}

// postgres
fn criterion_store_credit_card_postgres(c: &mut Criterion) {
    c.bench_function("store_postgres", |b| b.iter(|| store_credit_card_postgres()));
}

fn criterion_retrieve_credit_card_postgres(c: &mut Criterion) {
    c.bench_function("retrieve_postgres", |b| b.iter(|| retrieve_credit_card_postgres()));
}

fn criterion_store_retrieve_credit_card_postgres(c: &mut Criterion) {
    c.bench_function("store_retrieve_postgres", |b| b.iter(|| store_retrieve_credit_card_postgres()));
}



criterion_group!(
    benches,
    // redis
    criterion_store_credit_card_redis,
    criterion_retrieve_credit_card_redis,
    criterion_store_retrieve_credit_card_redis,
    // postgres
    criterion_store_credit_card_postgres,
    criterion_retrieve_credit_card_postgres,
    criterion_store_retrieve_credit_card_postgres
);
criterion_main!(benches);