use credit_card::CreditCard;

pub trait Tokenizer {
    fn new() -> Self;
    fn generate(&self, credit_card: &CreditCard) -> String;
}