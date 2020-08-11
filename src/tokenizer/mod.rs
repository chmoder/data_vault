mod blake3_tokenizer;

use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use credit_card::CreditCard;

pub use blake3_tokenizer::Blake3Tokenizer;

pub trait Tokenizer {
    fn new() -> Self;
    fn generate(&self, credit_card: &CreditCard) -> String;

    /// creates a random salt with given length
    /// # Arguments
    /// * `length` - the length of string to return
    /// ```rust
    /// use data_vault::tokenizer::Tokenizer;
    /// use data_vault::tokenizer::Blake3Tokenizer;
    ///
    /// let salt = Blake3Tokenizer::generate_salt(32);
    /// ```
    fn generate_salt(length: usize) -> String {
        let mut rng = thread_rng();
        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(length)
            .collect()
    }
}