use core::pipeline::Behaviour;
use rust_stemmers::Stemmer;

pub struct StemmingBehaviour {
    stemmer: Stemmer,
}

impl StemmingBehaviour {
    pub fn new(stemmer: Stemmer) -> Self {
        StemmingBehaviour { stemmer }
    }
}

impl Behaviour for StemmingBehaviour {
    fn execute(&self, token: String) -> Option<String> {
        Some(self.stemmer.stem(&token).to_string())
    }
}
