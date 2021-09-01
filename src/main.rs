use std::sync::Arc;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum Node<T> {
  Leaf {
    value: T,
  },
  Branch {
    left: Arc<Node<T>>,
    value: T,
    right: Arc<Node<T>>,
  },
}

impl<T> Hash for Node<T>
where
  T: Hash,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      Node::Leaf { value } => {
        value.hash(state);
      }
      Node::Branch { value, right, left } => {
        value.hash(state);
        right.hash(state);
        left.hash(state);
      }
    }
  }
}

impl<T> PartialEq for Node<T>
where
  T: PartialEq,
{
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Node::Leaf { value: l }, Node::Leaf { value: r }) => l == r,
      (
        Node::Branch {
          value: l,
          left: l1,
          right: r1,
        },
        Node::Branch {
          value: r,
          left: l2,
          right: r2,
        },
      ) => l == r && l1 == l2 && r1 == r2,
      _ => false,
    }
  }
}

impl<'a, T: Clone> Node<T>
where
  T: PartialOrd + 'a,
{
  pub fn from_vec(values: Vec<T>) -> Node<T> {
    if values.len() == 1 {
      Self::of_leaf(values[0].clone())
    } else {
      let len = values.len() / 2;
      let (left, right) = values.split_at(len);
      let (head, tail) = right.split_first().unwrap();
      Self::of_branch(
        Self::from_vec(left.to_vec()),
        head.clone(),
        Self::from_vec(tail.to_vec()),
      )
    }
  }

  pub fn of_leaf(value: T) -> Node<T> {
    Self::Leaf { value }
  }

  pub fn of_branch(left: Node<T>, value: T, right: Node<T>) -> Node<T> {
    Self::Branch {
      value,
      left: Arc::new(left),
      right: Arc::new(right),
    }
  }

  pub fn find(&'a self, value: T) -> Option<&'a Node<T>> {
    match self {
      Node::Leaf { value: v } if *v == value => Some(self),
      Node::Branch { value: v, .. } if *v == value => Some(self),
      Node::Branch {
        value: v, left: l, ..
      } if value < *v => l.find(value),
      Node::Branch {
        value: v, right: r, ..
      } if value > *v => r.find(value),
      _ => None,
    }
  }

  pub fn map<B, F>(&'a self, mut f: F) -> Node<B>
  where
    F: FnMut(&'a T) -> B,
  {
    match self {
      Node::Leaf { value } => {
        let new_value = f(value);
        Node::Leaf { value: new_value }
      }
      Node::Branch { value, left, right } => {
        let new_value = f(value);
        let new_left = Arc::new(left.map(|e| f(e)));
        let new_right = Arc::new(right.map(|e| f(e)));
        Node::Branch {
          value: new_value,
          left: new_left,
          right: new_right,
        }
      }
    }
  }

  pub fn value(&self) -> &T {
    match self {
      Node::Leaf { value } => value,
      Node::Branch { value, .. } => value,
    }
  }

  pub fn size(&self) -> usize {
    match self {
      Node::Leaf { .. } => 1,
      Node::Branch { left, right, .. } => 1 + left.size() + right.size(),
    }
  }

  pub fn max(&self) -> &T {
    match self {
      Node::Leaf { value } => value,
      Node::Branch { right, .. } => right.max(),
    }
  }

  pub fn min(&self) -> &T {
    match self {
      Node::Leaf { value } => value,
      Node::Branch { left, .. } => left.min(),
    }
  }
}

fn main() {
  let values = (1..=15).into_iter().collect::<Vec<_>>();
  let node = Node::from_vec(values);
  println!("node = {:?}", node);
  println!("node.size() = {}", node.size());
  println!("find(6) = {:?}", node.find(6).unwrap());
  println!("max = {}", node.max());
  println!("max = {}", node.max());
  println!("min = {}", node.min());
}
