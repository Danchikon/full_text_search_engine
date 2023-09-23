use crate::PipelineBehaviour;

pub struct Pipeline {
    behaviours: Vec<Box<dyn PipelineBehaviour>>
}

impl Pipeline {
    pub fn new(behaviours: Vec<Box<dyn PipelineBehaviour>>) -> Self {
        Pipeline { behaviours }
    }

    pub fn execute(&self, token: &String) -> Option<String> {
        let mut result = Some(token.clone());

        for behaviour in self.behaviours.iter() {
            result = match result {
                Some(value) => behaviour.execute(&value),
                None => None
            };
        }

        result
    }
}