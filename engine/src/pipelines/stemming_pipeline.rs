use core::PipelineBehaviour;

use rust_stemmers::Stemmer;

pub struct StemmingPipelineBehaviour {
    stemmer: Stemmer,
}

impl StemmingPipelineBehaviour {
    pub fn new(stemmer: Stemmer) -> Self {
        StemmingPipelineBehaviour { stemmer }
    }
}

impl PipelineBehaviour for StemmingPipelineBehaviour {
    fn execute(&self, token: &String) -> Option<String> {
        Some(self.stemmer.stem(token).to_string())
    }
}
