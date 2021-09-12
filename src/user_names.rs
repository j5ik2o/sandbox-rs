#[derive(Debug, Clone)]
pub struct UserName(String);

impl UserName {
  pub fn new(value: &str) -> Self {
    Self(value.to_owned())
  }
}

#[derive(Debug, Clone)]
pub struct UserNames(Vec<UserName>);

impl UserNames {
  pub fn new() -> Self {
    Self(Vec::new())
  }

  pub fn add_user_name0(&mut self, user_name: UserName) {
    self.0.push(user_name);
  }

  pub fn add_user_name1(&mut self, user_name: UserName) {
    println!("{:?}", user_name);
    println!("{:?}", user_name);
  }

  pub fn add_user_name2(mut self, user_name: UserName) {
    self.0.push(user_name);
  }
}

#[cfg(test)]
mod tests {
  use crate::user_names::{UserName, UserNames};

  #[test]
  fn test_add_user_name() {
    let mut uns = UserNames::new();
    {
      let un1 = UserName::new("test");
      uns.add_user_name0(un1);
      // ムーブするので使えない
    }
    {
      let un1 = UserName::new("test");
      uns.add_user_name0(un1.clone());
      println!("{:?}", un1)
    }
  }
}
