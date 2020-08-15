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

#[cfg(test)]
mod test {
    use crate::utils::Salt;

    #[test]
    fn test_salt_generate() {
        let salt = Salt::generate(12);
        assert_eq!(salt.len(), 12)
    }
}