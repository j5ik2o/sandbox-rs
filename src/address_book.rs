use anyhow::Result;
use std::fmt::Formatter;
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq)]
pub struct AddressEntryId(pub(crate) u64);

impl AddressEntryId {
  pub(crate) fn new(value: u64) -> Self {
    Self(value)
  }
}

#[derive(Debug, Clone)]
pub struct PersonName {
  pub first_name: String,
  pub last_name: String,
}

impl PersonName {
  pub fn new(first_name: &str, last_name: &str) -> Self {
    Self {
      first_name: first_name.to_owned(),
      last_name: last_name.to_owned(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Address {
  pub postal_code: String,
  pub pref: String,
  pub address: String,
  pub building: Option<String>,
}

impl Address {
  pub fn new(postal_code: &str, pref: &str, address: &str, building: Option<&str>) -> Self {
    Self {
      postal_code: postal_code.to_owned(),
      pref: pref.to_owned(),
      address: address.to_owned(),
      building: building.map(|e| e.to_owned()),
    }
  }
}

#[derive(Debug, Clone)]
pub struct AddressEntry {
  pub id: AddressEntryId,
  pub name: PersonName,
  pub address: Address,
}

impl AddressEntry {
  pub fn new(id: AddressEntryId, name: PersonName, address: Address) -> Self {
    Self { id, name, address }
  }
}

#[derive(Debug, Clone)]
pub struct AddressBook {
  entries: Vec<AddressEntry>,
}

impl Default for AddressBook {
  fn default() -> Self {
    AddressBook {
      entries: Vec::default(),
    }
  }
}

impl AddressBook {
  pub fn add_entry(&mut self, address_entry: AddressEntry) {
    self.entries.push(address_entry);
  }

  pub fn add_entries(&mut self, address_entries: &[AddressEntry]) {
    address_entries
      .to_vec()
      .into_iter()
      .for_each(|e| self.add_entry(e))
  }

  pub fn remove_entry(&mut self, address_entry_id: AddressEntryId) -> AddressEntry {
    let index = self
      .entries
      .iter()
      .position(|e| e.id == address_entry_id)
      .unwrap();
    self.entries.remove(index)
  }

  pub fn remove_entries(&mut self, address_entry_ids: &[AddressEntryId]) -> Vec<AddressEntry> {
    address_entry_ids
      .to_vec()
      .into_iter()
      .fold(vec![], |mut acc, address_entry_id| {
        acc.push(self.remove_entry(address_entry_id));
        acc
      })
  }

  pub fn iter(&self) -> impl Iterator<Item = &AddressEntry> {
    self.entries.iter()
  }
}

#[cfg(test)]
mod test {
  use crate::address_book::{AddressBook, AddressEntry, AddressEntryId, PersonName, Address};

  #[test]
  fn test_address_book() {
    let address_entry_id = AddressEntryId::new(1);
    let personal_name = PersonName::new("Junichi", "Kato");
    let address = Address::new("111-0001", "Tokyo-to", "minato-ku", Some("hoge building"));
    let address_entry = AddressEntry::new(address_entry_id, personal_name, address);
    let mut address_book = AddressBook::default();
    address_book.add_entry(address_entry);
    println!("{:?}", address_book);
  }
}
