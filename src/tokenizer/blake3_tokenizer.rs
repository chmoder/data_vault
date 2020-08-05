use credit_card::CreditCard;
use crate::tokenizer::{Tokenizer};

pub struct Blake3Tokenizer;
impl Tokenizer for Blake3Tokenizer {
    fn new() -> Self {
        Self {}
    }
    /// creates a token for a given credit card
    /// # Arguments
    /// * `CreditCard` - a credit card object to hash
    /// # Examples
    /// ```rust
    /// use data_vault::tokenizer::Tokenizer;
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
    /// let tokenizer = Blake3Tokenizer::new();
    /// let token = tokenizer.generate(&cc);
    ///
    /// ```
    fn generate(self: &Self, credit_card: &CreditCard) -> String {
        let salt = Self::generate_salt(32);

        let security_code = credit_card.security_code.clone();

        let mut hasher = blake3::Hasher::new();
        hasher.update(credit_card.number.as_bytes());
        hasher.update(credit_card.cardholder_name.as_bytes());
        hasher.update(credit_card.expiration_month.as_bytes());
        hasher.update(credit_card.expiration_year.as_bytes());
        hasher.update(security_code.unwrap_or_default().as_bytes());
        hasher.update(salt.as_bytes());

        let digest = hasher.finalize();
        let hex_digest = digest.to_hex();
        hex_digest.to_string()
    }
}