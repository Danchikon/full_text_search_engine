use std::collections::HashMap;

#[derive(Debug, Clone)]
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

    pub fn count(&self) -> usize {
        let mut count = 1;

        for child in self.children.values().into_iter() {
            count += child.count()
        }

        count
    }

    pub fn height(&self) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_should_create_root_node() {
        //Arrange
        let node = Node::root();

        //Assert
        assert!(node.is_root());
        assert!(node.is_leaf());
        assert!(!node.is_value());
        assert_eq!(node.count(), 1);
        assert_eq!(node.height(), 1);
    }

    #[test]
    fn new_should_create_node() {
        //Arrange
        let key = 'a';
        let is_value = true;

        let node = Node::new(key, is_value);

        //Assert
        assert!(!node.is_root());
        assert!(node.is_leaf());
        assert!(node.is_value());
        assert_eq!(node.count(), 1);
        assert_eq!(node.height(), 1);
    }

    #[test]
    fn count_should_return_count_of_nodes() {
        //Arrange
        let mut root = Node::root();
        let node_1 = Node::new('a', true);
        let mut node_2 = Node::new('b', false);
        let node_3 = Node::new('c', true);

        //Act
        node_2.add(node_3).unwrap();
        root.add(node_1).unwrap();
        root.add(node_2).unwrap();

        //Assert
        assert_eq!(root.count(), 4);
    }

    #[test]
    fn height_should_return_height_of_trie() {
        //Arrange
        let mut root = Node::root();
        let node_1 = Node::new('a', true);
        let mut node_2 = Node::new('b', false);
        let node_3 = Node::new('c', true);

        //Act
        node_2.add(node_3).unwrap();
        root.add(node_1).unwrap();
        root.add(node_2).unwrap();

        //Assert
        assert_eq!(root.height(), 3);
    }

    #[test]
    fn add_should_return_ok_when_node_is_not_root() {
        //Arrange
        let key = 'a';
        let is_value = true;

        let mut root = Node::root();
        let node = Node::new(key, is_value);

        //Act
        let add_result = root.add(node);

        //Assert
        assert!(add_result.is_ok());
    }

    #[test]
    fn add_should_return_err_when_node_is_root() {
        //Arrange
        let mut root = Node::root();
        let node = Node::root();

        //Act
        let add_result = root.add(node);

        //Assert
        assert!(add_result.is_err());
    }

    #[test]
    fn exists_should_return_true_when_value_is_exist() {
        //Arrange
        let value = "satisfy".to_string();

        let mut root = Node::root();

        //Act
        root.insert(value.clone());

        //Assert
        assert!(root.exists(&value));
    }

    #[test]
    fn exists_should_return_false_when_value_is_not_exist() {
        //Arrange
        let value = "satisfy".to_string();
        let value_to_search = "opportunity".to_string();

        let mut root = Node::root();

        //Act
        root.insert(value);

        //Assert
        assert!(!root.exists(&value_to_search));
    }
}
