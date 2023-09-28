use core::pipeline::Behaviour;

pub struct LowerCaseBehaviour;

impl LowerCaseBehaviour {
    pub fn new() -> Self {
        LowerCaseBehaviour
    }
}

impl Behaviour for LowerCaseBehaviour {
    fn execute(&self, token: String) -> Option<String> {
        Some(token.to_lowercase())
    }
}
