use bst::*;

#[test]
fn simple_bst_new() {
  let tree = SimpleBST::new();
  assert_eq!(tree.size(), 0);
}

#[test]
fn simple_bst_insert() {
  let mut tree = SimpleBST::new();
  tree.insert(5);
  assert_eq!(tree.size(), 1);
}

#[test]
fn simple_bst_inorder() {
  let mut tree = SimpleBST::new();
  tree.insert(5);
  tree.insert(1);
  tree.insert(3);
  let inorder = tree.get_inorder();
  let expected = vec![1, 3, 5];
  assert_eq!(inorder, expected);
}