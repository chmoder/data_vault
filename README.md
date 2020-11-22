# Data Vault


Data Vault is a library for storing and retrieving Credit Card data via Tokens.

[![Actions Status](https://github.com/chmoder/data_vault/workflows/Rust/badge.svg)](https://github.com/chmoder/data_vault/actions)
[![codecov](https://codecov.io/gh/chmoder/data_vault/branch/master/graph/badge.svg)](https://codecov.io/gh/chmoder/data_vault)
[![Crates.io](https://img.shields.io/crates/v/data_vault)](https://crates.io/crates/data_vault)
[![Documentation](https://docs.rs/data_vault/badge.svg)](https://docs.rs/data_vault)
[![License](https://img.shields.io/crates/l/data_vault.svg)](https://img.shields.io/crates/l/data_vault.svg)
[![Criterion](https://img.shields.io/criterion/chmoder/data_vault.svg)](https://criterion.dev/)


```toml
# Cargo.toml
[dependencies]
data_vault = "^0.3"
```

```dotenv
# .env OR environment variables
REDIS_URL=redis://:foobared@127.0.0.1/
# REDIS_POOL_MAX_SIZE=16
ENCRYPTED_DATA_VAULT_KEY=000102030405060708090a0b0c0d0e0f
ENCRYPTED_DATA_VAULT_IV=f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff
```

```rust
// example.rs

// traits
use data_vault::DataVault;
use data_vault::encryption::traits::Encryption;

// Interchangeable backend
use data_vault::RedisDataVault;
// Interchangeable encryption
use data_vault::encryption::AesGcmSivEncryption;
// Interchangeable tokenizer
use data_vault::tokenizer::Blake3Tokenizer;

// credit card type
use credit_card::CreditCard;

use tokio;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
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
```

# Current Features
- Store [Credit Cards](https://github.com/chmoder/credit_card)
- Store `String`
- Automatic Encryption and Decryption
- Blake3 tokenization
- Redis pool
- Postgres pool
- Configurable from .env file or Environment Variables
- Interchangeable Backend
- Interchangeable Encryption
- Interchangeable Tokenization hasher


# Performance (AMD Ryzen 9 3900X)
## Redis
This [example](https://github.com/chmoder/data_vault/blob/master/examples/redis_benchmark.rs) output the following performance stats
Tokenized **~100,000** credit cards per second.
```
tokenized and stored 100000 credit cards in 1.058474365s
retrieved 100000 credit cards in 5.353857633s
tokenized, stored, and retrieved 100000 credit cards in 6.412331998s
```

## Postgres
This [example](https://github.com/chmoder/data_vault/blob/master/examples/postgres_benchmark.rs) output the following performance stats
Tokenized **~1,500** credit cards per second.
```
tokenized and stored 1000 credit cards in 635.963241ms
retrieved 1000 credit cards in 938.834896ms
tokenized, stored, and retrieved 1000 credit cards in 1.574798137s
```