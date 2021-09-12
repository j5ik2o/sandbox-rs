use anyhow::Result;
use anyhow::anyhow;

pub trait Entity<ID>
where
  ID: PartialEq,
{
  fn id(&self) -> &ID;
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ThreadId(u64);

impl ThreadId {
  pub fn new(value: u64) -> Self {
    Self(value)
  }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MemberId(u64);

impl MemberId {
  pub fn new(value: u64) -> Self {
    Self(value)
  }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MemberName(String);

impl MemberName {
  pub fn new(value: String) -> Self {
    Self(value)
  }
}

#[derive(Debug, Clone)]
pub struct Member {
  id: MemberId,
  name: MemberName,
}

impl Member {
  pub fn new(id: MemberId, name: MemberName) -> Self {
    Self { id, name }
  }
}

impl Entity<MemberId> for Member {
  fn id(&self) -> &MemberId {
    &self.id
  }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MessageId(u64);

impl MessageId {
  pub fn new(value: u64) -> Self {
    Self(value)
  }
}

#[derive(Debug, Clone)]
pub struct MessageText(String);

impl MessageText {
  pub fn new(value: String) -> Self {
    Self(value)
  }
}

#[derive(Debug, Clone)]
pub struct Message {
  id: MessageId,
  sender_id: MemberId,
  text: MessageText,
}

impl Message {
  pub fn new(id: MessageId, sender_id: MemberId, text: MessageText) -> Self {
    Self {
      id,
      sender_id,
      text,
    }
  }
}

impl Entity<MessageId> for Message {
  fn id(&self) -> &MessageId {
    &self.id
  }
}

#[derive(Debug, Clone)]
pub struct Thread {
  id: ThreadId,
  members: Vec<Member>,
  messages: Vec<Message>,
}

impl Thread {
  pub fn new(id: ThreadId, members: &[Member], messages: &[Message]) -> Self {
    Self {
      id,
      members: Vec::from(members),
      messages: Vec::from(messages),
    }
  }

  fn is_member_id(&self, member_id: &MemberId) -> bool {
    self.members.iter().any(|m| m.id == *member_id)
  }

  pub fn add_message(&mut self, message: Message) -> Result<()> {
    if !self.is_member_id(&message.sender_id) {
      return Err(anyhow!("invalid member id: {:?}", message.sender_id));
    }
    self.messages.push(message);
    Ok(())
  }

  pub fn add_messages(&mut self, messages: &[Message]) -> Result<()> {
    for message in messages.to_vec() {
      let result = self.add_message(message);
      if result.is_err() {
        return result;
      }
    }
    Ok(())
  }

  pub fn remove_messages(&mut self, message_ids: &[MessageId]) {
    Self::convert_to_idx(&self.messages, message_ids)
      .into_iter()
      .for_each(|index| {
        self.messages.remove(index);
      });
  }

  pub fn messages(&self) -> &[Message] {
    &self.messages
  }

  pub fn add_members(&mut self, members: &[Member]) {
    self.members.extend_from_slice(members)
  }

  pub fn remove_members(&mut self, member_ids: &[MemberId]) {
    Self::convert_to_idx(&self.members, member_ids)
      .into_iter()
      .for_each(|index| {
        self.members.remove(index);
      });
  }

  fn convert_to_idx<ID, M>(members: &Vec<M>, member_ids: &[ID]) -> Vec<usize>
  where
    ID: PartialEq,
    M: Entity<ID>,
  {
    members
      .iter()
      .enumerate()
      .filter(|(_, member)| member_ids.contains(member.id()))
      .map(|(index, _)| index)
      .collect::<Vec<_>>()
  }

  pub fn members(&self) -> &[Member] {
    &self.members
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_thread() {
    let thread_id = ThreadId::new(1);
    let mut thread = Thread::new(thread_id, &[], &[]);
    let member_id = MemberId::new(1);
    let member_name = MemberName::new("member_1".to_owned());
    let members = [Member::new(member_id.clone(), member_name)];
    thread.add_members(&members);
    let message_id = MessageId::new(1);
    let messages = [Message::new(
      message_id,
      member_id,
      MessageText::new("".to_owned()),
    )];
    thread.add_messages(&messages).unwrap();
    println!("members = {:?}", thread.members());
    println!("messages = {:?}", thread.messages());
  }
}
