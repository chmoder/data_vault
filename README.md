# Data Vault


Data Vault is a library for storing and retrieving Credit Card data via Tokens.

[![Actions Status](https://github.com/chmoder/data_vault/workflows/Rust/badge.svg)](https://github.com/chmoder/data_vault/actions)
[![codecov](https://codecov.io/gh/chmoder/data_vault/branch/master/graph/badge.svg)](https://codecov.io/gh/chmoder/data_vault)
[![crates.io](https://meritbadge.herokuapp.com/data_vault)](https://crates.io/crates/data_vault)
[![Documentation](https://docs.rs/data_vault/badge.svg)](https://docs.rs/data_vault)
[![License](https://img.shields.io/crates/l/data_vault.svg)](https://img.shields.io/crates/l/data_vault.svg)


```toml
# Cargo.toml
[dependencies]
data_vault = "^0.2"
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

// data vault
use data_vault::RedisDataVault;
// Interchangable encryption
use data_vault::encryption::AesGcmSivEncryption;
// Interchangable tokenizer
use data_vault::tokenizer::Blake3Tokenizer;

// credit card type
use credit_card::CreditCard;

use tokio;

#[tokio::main(core_threads = 4)]
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
- Redis Server, URL connection configuration
- Configurable from .env file or Environment Variables
- Interchangable Encryption
- Interchangable Tokenization hasher

# Future Features
- Postgres Database

# Performance
This [example](https://github.com/chmoder/data_vault/blob/master/examples/benchmark.rs) output the following performance stats with an AMD Ryzen 9 3900X.
Showing the possibility of tokenizing **~100,000** credit cards per second.
```
tokenized and stored 100000 credit cards in 1.058474365s
retrieved 100000 credit cards in 5.353857633s
tokenized, stored, and retrieved 100000 credit cards in 6.412331998s
```

### Notice:
This is under development right now, so interfaces
and apis will be changing.  If you are interested
in using this please create an issue or reach out
with your feature request so I can help add it.