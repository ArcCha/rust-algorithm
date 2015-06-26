#[cfg(test)]
mod tests;

pub trait BST {
  fn insert(&mut self, x: i32);

  fn get_inorder(&self) -> Vec<i32>;
}

pub struct SimpleBST {
  root: Box<Node>,
  size: i32
}

#[derive(PartialEq)]
enum Node {
  Node {
    key:   i32,
    parent: Box<Node>,
    left:   Box<Node>,
    right:  Box<Node>
  },
  Null
}

impl SimpleBST {
  pub fn new() -> SimpleBST {
    SimpleBST { root : Box::new(Node::Null),
                size : 0 }
  }

  pub fn size(&self) -> i32 {
    self.size
  }

  fn get_inorder_aux(&self, curr: &Box<Node>, out: &mut Vec<i32>) {
    match **curr {
      Node::Null => (),
      Node::Node{ key, ref left, ref right, ..} => {
        self.get_inorder_aux(left, out);
        out.push(key);
        self.get_inorder_aux(right, out);
      }
    };
  }

  fn insert_aux(curr: &mut Box<Node>, x: i32) {
    match **curr {
      Node::Null => *curr = Box::new(Node::Node {
        key: x,
        parent: Box::new(Node::Null),
        left: Box::new(Node::Null),
        right: Box::new(Node::Null)
      }),
      Node::Node { key, ref mut left, ..} if x <= key => {
        SimpleBST::insert_aux(left, x);
      },
      Node::Node { key, ref mut right, ..} => {
        SimpleBST::insert_aux(right, x);
      }
    }
  }
}

impl BST for SimpleBST {
  fn insert(&mut self, x: i32) {
    self.size += 1;
    let r = &mut self.root;
    SimpleBST::insert_aux(r, x);
  }

  fn get_inorder(&self) -> Vec<i32> {
    let mut res = Vec::<i32>::new();
    self.get_inorder_aux(&self.root, &mut res);
    res
  }
}