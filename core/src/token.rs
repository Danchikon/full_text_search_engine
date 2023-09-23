pub struct Token {
    position: u64,
    data: String,
}

impl Token {
    pub fn new(data: String, position: u64) -> Self {
        Token { data, position }
    }

    pub fn position(self) -> u64 {
        self.position
    }

    pub fn data(self) -> String {
        self.data
    }
}
