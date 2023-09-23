use crate::Token;

pub trait Tokenizer {
    fn tokenize(self, text: String) -> Vec<Token>;
}
