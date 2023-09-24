use std::collections::HashMap;

pub struct Node {
    key: Option<char>,
    nodes: HashMap<char, Node>,
}

impl Node {
    pub fn new(key: char) -> Self {
        Node {
            key: Some(key),
            nodes: HashMap::new(),
        }
    }

    pub fn root() -> Self {
        Node {
            key: None,
            nodes: HashMap::new(),
        }
    }

    pub fn is_root(&self) -> bool {
        self.key.is_none()
    }

    pub fn add(&mut self, node: Node) -> Result<(), ()> {
        match node.key {
            Some(key) => {
                self.nodes.insert(key, node);

                Ok(())
            }
            None => Err(()),
        }
    }

    pub fn insert(&mut self, value: String) {
        if value.len() > 0 {
            let mut value = value.clone();
            let first = value.remove(0);

            if !self.nodes.contains_key(&first) {
                self.add(Node::new(first)).unwrap();
            }

            self.nodes.get_mut(&first).unwrap().insert(value);
        } else {
            return;
        }
    }
}
