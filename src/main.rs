#[macro_use]
extern crate tailcall;

use std::fmt::Formatter;

// #[tokio::main]
// async fn main() {
//   // Calling `say_world()` does not execute the body of `say_world()`.
//   let op = say_world();
//
//   // This println! comes first
//   println!("hello");
//
//   // Calling `.await` on `op` starts executing `say_world`.
//   op.await;
// }
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime;
use tokio::runtime::{Runtime, Builder};

use crate::address_book::AddressBook;
use crate::node::Node;
use crate::ref_node::RefNode;
use tokio::time::Duration;

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

#[tailcall]
fn gcd(a: u64, b: u64) -> u64 {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}
// #[tokio::main]
// pub async fn main() -> Result<()> {
//   // Open a connection to the mini-redis address.
//   let mut client = client::connect("127.0.0.1:6379").await?;
//
//   // Set the key "hello" with value "world"
//   client.set("hello", "world".into()).await?;
//
//   // Get key "hello"
//   let result = client.get("hello").await?;
//
//   println!("got value from the server; result={:?}", result);
//
//   Ok(())
// }
// async fn say_world() {
//   println!("world");
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let rt = Builder::new_multi_thread()
      .enable_all()
      .worker_threads(4)
      .thread_name("my-custom-name")
      .thread_stack_size(3 * 1024 * 1024)
      .build()?;

  Ok(rt.block_on(async {
    let h1 = tokio::spawn(async move {
      for i in 1..10 {
        let id = std::thread::current().id();
        println!("{:?}:1:i = {}", id, i);
        tokio::time::sleep(Duration::from_secs(1)).await;
      }
    });
    tokio::time::sleep(Duration::from_secs(1)).await;
    let h2 = tokio::spawn(async move {
      for i in 1..10 {
        let id = std::thread::current().id();
        println!("{:?}:2:i = {}",id, i);
        tokio::time::sleep(Duration::from_secs(1)).await;
      }
    });
    tokio::join!(h1, h2);
  }))


}


// #[tokio::main]
// async fn main() {
//   // Bind the listener to the address
//   let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
//
//   loop {
//     // The second item contains the IP and port of the new connection.
//     let (socket, _) = listener.accept().await.unwrap();
//     process(socket).await;
//   }
//}

// async fn process(socket: TcpStream) {
//   // The `Connection` lets us read/write redis **frames** instead of
//   // byte streams. The `Connection` type is defined by mini-redis.
//   let mut connection = Connection::new(socket);
//
//   if let Some(frame) = connection.read_frame().await.unwrap() {
//     println!("GOT: {:?}", frame);
//
//     // Respond with an error
//     let response = Frame::Error("unimplemented".to_string());
//     connection.write_frame(&response).await.unwrap();
//   }
// }

fn main1() {
  let a = gcd(1, 100);
  println!("{}", a);
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
