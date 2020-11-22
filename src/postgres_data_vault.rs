use async_trait::async_trait;
use credit_card::CreditCard;
use crate::traits::{DataVault, PoolErrors};
use crate::config::{DeadpoolPostgresConfig};
use crate::encryption::traits::Encryption;
use crate::tokenizer::{Tokenizer};
use deadpool_postgres::{tokio_postgres};
use std::error;

/// Use postgres as a data vault back end
///
/// This implementation uses deadpool_postgres
///
/// DDL
///
/// -- Drop table
///
/// -- DROP TABLE public.data_vault;
///
/// CREATE TABLE public.data_vault (
/// id bigserial NOT NULL DEFAULT nextval('data_vault_id_seq'::regclass),
/// "token" varchar(64) NOT NULL,
/// credit_card bytea NOT NULL
/// );
/// CREATE UNIQUE INDEX data_vault_token_idx ON public.data_vault USING btree (token);
///
///
/// Connection setup is available as environment
/// variables or a .env file with the following
/// options:
/// PG_HOST=127.0.0.1
/// PG_USER=data_vault
/// PG_PASSWORD=password
/// PG_DBNAME=data_vault
/// PG_POOL_MAX_SIZE=16
/// PG_POOL_TIMEOUTS_WAIT_SECS=5
/// PG_POOL_TIMEOUTS_WAIT_NANOS=0
///
/// # Examples
/// ```rust
/// use data_vault::{DataVault, PostgresDataVault};
/// use data_vault::encryption::AesGcmSivEncryption;
/// use data_vault::tokenizer::Blake3Tokenizer;
/// let data_vault = PostgresDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();
/// ```
pub struct PostgresDataVault<E, T> {
    pool: deadpool_postgres::Pool,
    encryption: E,
    tokenizer: T,
}

const SELECT_CREDIT_CARD: &str = "SELECT credit_card FROM data_vault WHERE token = $1";
const INSERT_CREDIT_CARD: &str = "INSERT INTO data_vault VALUES (token, credit_card) ($1, $2)";
const UPDATE_CREDIT_CARD: &str = "UPDATE data_vault SET credit_card = $2 WHERE token = $1";
const UPSERT_CREDIT_CARD: &str = "INSERT INTO data_vault (token, credit_card) VALUES ($1, $2) ON CONFLICT (token) DO UPDATE SET credit_card = EXCLUDED.credit_card";
const DELETE_CREDIT_CARD: &str = "DELETE FROM data_vault WHERE token = $1";

#[async_trait]
impl<E, T> DataVault for PostgresDataVault<E, T>
    where
        E: Encryption + std::marker::Sync + std::marker::Send,
        T: Tokenizer + std::marker::Sync + std::marker::Send,
{
    /// Create new PostgresDataVault backend
    /// # examples
    /// ```rust
    /// use data_vault::{DataVault, PostgresDataVault};
    /// use data_vault::encryption::AesGcmSivEncryption;
    /// use data_vault::tokenizer::Blake3Tokenizer;
    /// let data_vault = PostgresDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();
    /// ```
    fn new() -> Result<Self, Box<dyn error::Error>> {
        let cfg = DeadpoolPostgresConfig::from_env()?;

        let pool = cfg.postgres.create_pool(tokio_postgres::NoTls)?;

        let postgres_data_vault = PostgresDataVault {
            pool,
            encryption: E::new(),
            tokenizer: T::new()
        };

        Ok(postgres_data_vault)
    }

    /// Encrypt and Store a string with the given token as the postgres key
    /// Arguments:
    ///     * `CreditCard` - the cc object that you wish to store
    /// return:
    ///     the token as String
    /// # example
    /// ```rust
    /// use data_vault::{DataVault, PostgresDataVault};
    /// use data_vault::encryption::AesGcmSivEncryption;
    /// use data_vault::tokenizer::Blake3Tokenizer;
    ///
    /// let token = String::from("abc123");
    /// let credit_card_string = String::from("{number: 123}");
    /// let data_vault = PostgresDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();
    /// data_vault.store(&token, &credit_card_string);
    /// ```
    async fn store(&self, token: &String, string: &String) -> Result<(), PoolErrors> {
        let client = self.pool.get().await?;
        let encrypted_json = self.encryption.encrypt(string.as_bytes());
        let stmt = client.prepare(UPSERT_CREDIT_CARD).await.unwrap();
        let _ = client.query(&stmt, &[&token, &encrypted_json]).await.unwrap();
        Ok(())
    }

    /// Store the credit card in the data vault
    /// Arguments:
    ///     * `CreditCard` - the cc object that you wish to store
    /// return:
    ///     A new token as String
    /// # example
    /// ```rust
    /// use data_vault::{DataVault, PostgresDataVault};
    /// use data_vault::encryption::AesGcmSivEncryption;
    /// use data_vault::tokenizer::Blake3Tokenizer;
    /// use credit_card::CreditCard;
    ///
    /// let cc = CreditCard {
    ///    number: "4111111111111111".to_string(),
    ///    cardholder_name: "Graydon Hoare".to_string(),
    ///    expiration_month: "01".to_string(),
    ///    expiration_year: "2023".to_string(),
    ///    brand: None,
    ///    security_code: None
    /// };
    ///
    /// let data_vault = PostgresDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();
    /// let token = data_vault.store_credit_card(&cc);
    /// ```
    async fn store_credit_card(&self, credit_card: &CreditCard) -> Result<String, PoolErrors> {
        let token = self.tokenizer.generate(&credit_card);
        let credit_card_json = serde_json::to_string(&credit_card).unwrap();
        let _:() = self.store(&token, &credit_card_json).await?;
        Ok(token)
    }

    /// Get decrypted arbitrary data from the vault by token
    /// Arguments:
    ///     * `token`: the string form of the ID of the data
    /// returns:
    ///     * the decrypted string of data
    /// # example
    /// ```rust,ignore
    /// use data_vault::DataVault;
    /// use data_vault::PostgresDataVault;
    /// use data_vault::encryption::AesGcmSivEncryption;
    /// use data_vault::tokenizer::Blake3Tokenizer;
    ///
    /// let token = String::from("abc123");
    /// let cc_string = String::from("{number: 123}");
    /// let data_vault = PostgresDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();
    /// data_vault.store(&token, &cc_string).await;
    /// let credit_card_string = data_vault.retrieve(&token)
    /// ```
    async fn retrieve(&self, token: &String) -> Result<String, PoolErrors> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(SELECT_CREDIT_CARD).await.unwrap();
        let row_result = client.query_one(&stmt, &[&token]).await;
        let mut encrypted_credit_card_json: Vec<u8> = Vec::new();
        if row_result.is_ok() {
           let row = row_result.unwrap();
           encrypted_credit_card_json = row.get("credit_card");
        }
        Ok(self.encryption.decrypt(encrypted_credit_card_json.as_slice()))
    }

    /// Get the credit card from the data vault given a token
    /// Arguments:
    ///     * `token`: the string form of the ID of the data
    /// returns:
    ///     * `CreditCard` object
    /// # Example
    /// ```rust,ignore
    /// use data_vault::DataVault;
    /// use data_vault::PostgresDataVault;
    /// use data_vault::encryption::AesGcmSivEncryption;
    /// use data_vault::tokenizer::Blake3Tokenizer;
    /// use credit_card::CreditCard;
    ///
    /// let cc = CreditCard {
    ///    number: "4111111111111111".to_string(),
    ///    cardholder_name: "Graydon Hoare".to_string(),
    ///    expiration_month: "01".to_string(),
    ///    expiration_year: "2023".to_string(),
    ///    brand: None,
    ///    security_code: None
    /// };
    ///
    /// let data_vault = PostgresDataVault::<AesGcmSivEncryption, Blake3Tokenizer>::new().unwrap();
    /// let token = data_vault.store_credit_card(&cc).await;
    /// let credit_card = data_vault.retrieve_credit_card(&token).await;
    /// ```
    async fn retrieve_credit_card(&self, token: &String) -> Result<CreditCard, PoolErrors> {
        let credit_card_json = self.retrieve(token).await?;
        Ok(serde_json::from_str(&credit_card_json).unwrap_or_default())
    }
}