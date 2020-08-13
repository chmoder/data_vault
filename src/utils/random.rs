use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

pub struct Salt;
impl Salt {
    /// creates a random salt with given length
    /// # Arguments
    /// * `length` - the length of string to return
    /// ```rust
    /// use data_vault::utils::Salt;
    ///
    /// let salt = Salt::generate(32);
    /// ```
    pub fn generate(length: usize) -> String {
        let mut rng = thread_rng();
        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(length)
            .collect()
    }
}