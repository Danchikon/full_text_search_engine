use core::PipelineBehaviour;

pub struct LowerCasePipelineBehaviour;

impl LowerCasePipelineBehaviour {
    pub fn new() -> Self {
        LowerCasePipelineBehaviour
    }
}

impl PipelineBehaviour for LowerCasePipelineBehaviour {
    fn execute(&self, token: &String) -> Option<String> {
        Some(token.to_lowercase())
    }
}
