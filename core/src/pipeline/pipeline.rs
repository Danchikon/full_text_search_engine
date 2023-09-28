use super::Behaviour;

pub struct Pipeline {
    behaviours: Vec<Box<dyn Behaviour>>,
}

impl Pipeline {
    pub fn new(behaviours: Vec<Box<dyn Behaviour>>) -> Self {
        Pipeline { behaviours }
    }

    pub fn execute(&self, token: String) -> Option<String> {
        let mut result = Some(token);

        for behaviour in self.behaviours.iter() {
            result = match result {
                Some(value) => behaviour.execute(value),
                None => None,
            };
        }

        result
    }
}
