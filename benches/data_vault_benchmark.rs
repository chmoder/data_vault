use tokio;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use credit_card::CreditCard;
use data_vault::{RedisDataVault, DataVault};
use core::time::Duration;

#[tokio::main]
async fn store_retrieve_credit_card() {
    let vault = RedisDataVault::new();

    let cc = CreditCard {
        number: "4111111111111111".to_string(),
        cardholder_name: "Graydon Hoare".to_string(),
        expiration_month: "01".to_string(),
        expiration_year: "2023".to_string(),
        brand: None,
        security_code: None
    };

    let token = vault.store_credit_card(&cc).await;
    let credit_card = vault.retrieve_credit_card(&token.to_string()).await;
    assert_eq!(credit_card.number, cc.number)
}

#[tokio::main]
async fn store_credit_card() {
    let vault = RedisDataVault::new();

    let cc = CreditCard {
        number: "4111111111111111".to_string(),
        cardholder_name: "Graydon Hoare".to_string(),
        expiration_month: "01".to_string(),
        expiration_year: "2023".to_string(),
        brand: None,
        security_code: None
    };

    let token = vault.store_credit_card(&cc).await;
    assert_eq!(token.len(), 64)
}

#[tokio::main]
async fn retrieve_credit_card() {
    let token = "token";
    let vault = RedisDataVault::new();
    let credit_card = vault.retrieve_credit_card(&token.to_string()).await;

    let cc = CreditCard {
        number: "4111111111111111".to_string(),
        cardholder_name: "Graydon Hoare".to_string(),
        expiration_month: "01".to_string(),
        expiration_year: "2023".to_string(),
        brand: None,
        security_code: None
    };

    if credit_card.number.len() == 0 {
        vault.store_credit_card(&cc).await;
    }

    assert_eq!(credit_card.number, cc.number)
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


criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10).nresamples(2000).measurement_time(Duration::new(5, 0));
    targets = criterion_store_retrieve_credit_card
    //criterion_store, criterion_retrieve
}
criterion_main!(benches);