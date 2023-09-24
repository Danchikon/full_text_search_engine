pub trait PipelineBehaviour {
    fn execute(&self, token: String) -> Option<String>;
}
