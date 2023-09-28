#[derive(Debug, Clone)]
pub struct Token {
    position: usize,
    pub data: String,
}

impl Token {
    pub fn new(data: String, position: usize) -> Self {
        Token { data, position }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn data(&self) -> &String {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn count(&self) -> usize {
        self.data.chars().count()
    }
}
