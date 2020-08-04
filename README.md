# Data Vault


Data Vault is a library for storing and retrieving Credit Card data via Tokens.


#### add data_vault as a dependency to Cargo.toml 
```toml
data_vault = "0.1.0"
```

```rust,norun
use credit_card::CreditCard;
use data_vault::{RedisDataVault, DataVault};

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
let credit_card = vault.retrieve_credit_card(&token).await;
assert_eq!(credit_card.number, cc.number)
```

# Current Features
- Store `String`
- Store [Credit Cards](https://github.com/chmoder/credit_card)
- Automatic Encryption and Decryption
- Redis Database with Auth password
- Configurable from .env file or Environment Variables

# Future Features
- Swappable Encryption
- Swappable Tokenization hasher
- Postgres Database

### Notice:
This is under development right now, so interfaces
and apis will be changing.  If you are interested
in using this please create an issue or reach out
with your feature request so I can help add it.