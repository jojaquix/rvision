use crate::point::*;
use crate::rect::*;
use crate::screen::*;
use crate::view::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub type GroupLink = Rc<RefCell<dyn Group>>;
pub type OWGroupLink = Option<Weak<RefCell<dyn Group>>>;
pub type OGroupLink = Option<GroupLink>;

pub struct TGroup {
  pub name: String,
}

impl TGroup {
  pub fn new(name: String) -> GroupLink {
    Rc::new(RefCell::new(TGroup { name: name }))
  }
}

pub trait Group {
  fn get_name(&self) -> String;

  fn hello(&self) {
    print!("Hello from group trait {}", self.get_name())
  }
}

impl Group for TGroup {
  fn get_name(&self) -> String {
    self.name.clone()
  }
}
