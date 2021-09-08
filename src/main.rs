use std::fmt::Formatter;

use crate::node::Node;
use crate::ref_node::RefNode;
use crate::address_book::AddressBook;

pub mod address_book;
mod currency;
mod money;
mod node;
mod ref_node;
mod thread;
mod user_names;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Value(i32);

impl std::fmt::Display for Value {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

fn main() {
  let values = (1..=15).into_iter().map(|e| Value(e)).collect::<Vec<_>>();
  let node = Node::from_vec(&values);
  println!("node = {:?}", node);
  println!("node.size() = {}", node.size());
  println!("find(6) = {:?}", node.find(Value(6)).unwrap());
  println!("min = {}", node.as_min());
  // 実体が欲しい場合は呼び出し側が必要に応じてclone()する。
  let max = node.as_max().clone();
  println!("max = {}", max);

  let values = (1..=15).into_iter().map(|e| Value(e)).collect::<Vec<_>>();
  let node = RefNode::from_vec(&values);
  println!("node = {:?}", node);
  println!("node.size() = {}", node.size());
  println!("find(6) = {:?}", node.find(Value(6)).unwrap());
  println!("min = {}", node.as_min());
  // 実体が欲しい場合は呼び出し側が必要に応じてclone()する。
  let max = node.as_max().clone();
  println!("max = {}", max);
}
