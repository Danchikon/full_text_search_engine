use core::pipeline::Behaviour;
use std::collections::HashSet;

pub struct RedundantTokensBehaviour {
    redundant_tokens: HashSet<String>,
}

impl RedundantTokensBehaviour {
    pub fn new(redundant_tokens: HashSet<String>) -> Self {
        RedundantTokensBehaviour { redundant_tokens }
    }
}

impl Behaviour for RedundantTokensBehaviour {
    fn execute(&self, token: String) -> Option<String> {
        if self.redundant_tokens.contains(&token) {
            None
        } else {
            Some(token)
        }
    }
}
