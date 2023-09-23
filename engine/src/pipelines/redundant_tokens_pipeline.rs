use core::PipelineBehaviour;
use std::collections::HashSet;

pub struct RedundantTokensPipelineBehaviour {
    redundant_tokens: HashSet<String>,
}

impl RedundantTokensPipelineBehaviour {
    pub fn new(redundant_tokens: HashSet<String>) -> Self {
        RedundantTokensPipelineBehaviour { redundant_tokens }
    }
}

impl PipelineBehaviour for RedundantTokensPipelineBehaviour {
    fn execute(&self, token: &String) -> Option<String> {
        if self.redundant_tokens.contains(token) {
            None
        } else {
            Some(token.clone())
        }
    }
}
