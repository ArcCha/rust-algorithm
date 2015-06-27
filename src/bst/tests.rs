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

#[test]
fn simple_bst_search() {
  let mut tree = SimpleBST::new();
  tree.insert(5);
  tree.insert(7);
  tree.insert(2);
  tree.insert(1);
  tree.insert(8);
  assert_eq!(tree.search(1), Some(1));
  assert_eq!(tree.search(10), None);
}

use test::Bencher;
use rand::Rng;
use rand::thread_rng;

#[bench]
fn bench_simple_bst_insert(b: &mut Bencher) {
  let mut tree = SimpleBST::new();
  let mut rng = thread_rng();
  b.iter(|| tree.insert(rng.gen()));
}

#[bench]
fn bench_simple_bst_search(b: &mut Bencher) {
  let mut tree = SimpleBST::new();
  let mut rng = thread_rng();
  for i in 1..1_000 {
    tree.insert(rng.gen());
  }
  b.iter(|| tree.search(rng.gen()));
}