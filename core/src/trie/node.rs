use std::collections::HashMap;

pub struct Node {
    key: Option<char>,
    is_value: bool,
    children: HashMap<char, Node>,
}

impl Node {
    pub fn new(key: char, is_value: bool) -> Self {
        Node {
            key: Some(key),
            is_value,
            children: HashMap::new(),
        }
    }

    pub fn root() -> Self {
        Node {
            key: None,
            is_value: false,
            children: HashMap::new(),
        }
    }

    pub fn is_root(&self) -> bool {
        self.key.is_none()
    }

    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    pub fn is_value(&self) -> bool {
        self.is_value
    }

    pub fn add(&mut self, node: Self) -> Result<(), ()> {
        match node.key {
            Some(key) => {
                self.children.insert(key, node);

                Ok(())
            }
            None => Err(()),
        }
    }

    pub fn insert(&mut self, mut value: String) {
        let len = value.len();

        if len > 0 {
            let is_value = len == 1;
            let first = value.remove(0);

            match self.children.get_mut(&first) {
                Some(child) => {
                    child.is_value = is_value;
                    child.insert(value);
                }
                None => {
                    let mut node = Self::new(first, is_value);

                    node.insert(value);
                    self.add(node).unwrap();
                }
            }
        }
    }

    pub fn count(&self) -> u32 {
        let mut count = 1;

        for child in self.children.values().into_iter() {
            count += child.count()
        }

        count
    }

    pub fn height(&self) -> u32 {
        let mut height = 0;

        for child in self.children.values().into_iter() {
            height = height.max(child.height())
        }

        height + 1
    }

    pub fn child(&self, key: &char) -> Option<&Node> {
        self.children.get(key)
    }

    pub fn values(&self) -> Vec<String> {
        let mut values = Vec::<String>::new();

        self.values_internal(String::new(), &mut values);

        values
    }

    fn values_internal(&self, current: String, values: &mut Vec<String>) {
        for (key, child) in self.children.iter() {
            let mut next = current.clone();
            next.push(key.clone());

            child.values_internal(next, values)
        }

        if self.is_value() {
            values.push(current);
        }
    }

    pub fn exists(&self, value: &String) -> bool {
        let mut current = Some(self);

        for key in value.chars().into_iter() {
            match current {
                Some(node) => current = node.children.get(&key),
                None => return false,
            }
        }

        current.is_some_and(|node| node.is_value())
    }
}

