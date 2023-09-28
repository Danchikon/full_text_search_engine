pub trait Behaviour {
    fn execute(&self, token: String) -> Option<String>;
}
