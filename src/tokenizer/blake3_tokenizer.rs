use credit_card::CreditCard;
use crate::tokenizer::{Tokenizer};

pub struct Blake3Tokenizer;
impl Tokenizer for Blake3Tokenizer {
    fn new() -> Self {
        Self {}
    }
    /// creates a token for a given credit card
    /// # Arguments
    /// * `credit_card` - a credit card object to hash
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