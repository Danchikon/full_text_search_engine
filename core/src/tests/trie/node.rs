use crate::trie::Node;


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
