use std::sync::Arc;
use std::hash::{Hash, Hasher};
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum RefNode<'a, T> {
  Leaf {
    value: &'a T,
  },
  Branch {
    left: Arc<RefNode<'a, T>>,
    value: &'a T,
    right: Arc<RefNode<'a, T>>,
  },
}

impl<'a, T> Hash for RefNode<'a, T>
where
  T: Hash,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      RefNode::Leaf { value } => {
        value.hash(state);
      }
      RefNode::Branch { value, right, left } => {
        value.hash(state);
        right.hash(state);
        left.hash(state);
      }
    }
  }
}

impl<'a, T> PartialEq for RefNode<'a, T>
where
  &'a T: PartialEq,
{
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (RefNode::Leaf { value: l }, RefNode::Leaf { value: r }) => l == r,
      (
        RefNode::Branch {
          value: l,
          left: l1,
          right: r1,
        },
        RefNode::Branch {
          value: r,
          left: l2,
          right: r2,
        },
      ) => l == r && l1 == l2 && r1 == r2,
      _ => false,
    }
  }
}

impl<'a, T: Clone> RefNode<'a, T>
where
  &'a T: PartialOrd,
  T: PartialOrd,
{
  pub fn from_vec(values: &'a [T]) -> RefNode<'a, T> {
    if values.len() == 1 {
      let v = &values[0];
      Self::of_leaf(v)
    } else {
      let len = values.len() / 2;
      let (left, right) = values.split_at(len);
      let (head, tail) = right.split_first().unwrap();
      Self::of_branch(Self::from_vec(left), head, Self::from_vec(tail))
    }
  }

  pub fn of_leaf(value: &'a T) -> RefNode<'a, T> {
    Self::Leaf { value }
  }

  pub fn of_branch(left: RefNode<'a, T>, value: &'a T, right: RefNode<'a, T>) -> RefNode<'a, T> {
    Self::Branch {
      value,
      left: Arc::new(left),
      right: Arc::new(right),
    }
  }

  pub fn find(&'a self, value: T) -> Option<&'a RefNode<T>> {
    match self {
      RefNode::Leaf { value: v } if **v == value => Some(self),
      RefNode::Branch { value: v, .. } if **v == value => Some(self),
      RefNode::Branch {
        value: v, left: l, ..
      } if value < **v => l.find(value),
      RefNode::Branch {
        value: v, right: r, ..
      } if value > **v => r.find(value),
      _ => None,
    }
  }

  pub fn map<'b, B, F>(&'a self, mut f: F) -> RefNode<'b, B>
  where
    F: FnMut(&'a T) -> &'b B,
    'b: 'a,
  {
    match self {
      RefNode::Leaf { value } => {
        let new_value = f(value);
        RefNode::Leaf { value: new_value }
      }
      RefNode::Branch { value, left, right } => {
        let new_value = f(value);
        let new_left = Arc::new(left.map(|e| f(e)));
        let new_right = Arc::new(right.map(|e| f(e)));
        RefNode::Branch {
          value: new_value,
          left: new_left,
          right: new_right,
        }
      }
    }
  }

  // (1) 値を読みたいだけなら通常はこれでよい。
  // 実体が欲しいなら呼び出し先が必要に応じてcloneを呼び出す
  pub fn as_value(&self) -> &T {
    match self {
      &RefNode::Leaf { value } => value,
      &RefNode::Branch { value, .. } => value,
    }
  }

  // (2) 効率が悪い実装。
  // cloneするかどうかは呼び出し側で決める。できるだけ遅延させよう。
  pub fn to_value_clone(&self) -> T {
    match self {
      &RefNode::Leaf { value } => value.clone(),
      &RefNode::Branch { value, .. } => value.clone(),
    }
  }

  // (3) 無駄なself消費,cloneも遅延すべき
  pub fn to_value_move1(self) -> T {
    match self {
      RefNode::Leaf { value } => value.clone(),
      RefNode::Branch { value, .. } => value.clone(),
    }
  }

  // (4) 実体版と違ってコンパイル可能だが、無駄なself消費。
  pub fn to_value_move2(self) -> &'a T {
    match self {
      RefNode::Leaf { value } => value,
      RefNode::Branch { value, .. } => value,
    }
  }

  pub fn size(&self) -> usize {
    match self {
      RefNode::Leaf { .. } => 1,
      RefNode::Branch { left, right, .. } => 1 + left.size() + right.size(),
    }
  }

  pub fn as_max(&self) -> &T {
    match self {
      RefNode::Leaf { value } => value,
      RefNode::Branch { right, .. } => right.as_max(),
    }
  }

  pub fn as_min(&self) -> &T {
    match self {
      RefNode::Leaf { value } => value,
      RefNode::Branch { left, .. } => left.as_min(),
    }
  }
}
