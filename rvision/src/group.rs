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
  view: TView,
  name: String,
}

impl TGroup {
  pub fn new(name: String, rect : TRect, owner: OWGroupLink) -> GroupLink {
    Rc::new(RefCell::new(TGroup { name: name, view: TView::new(rect, owner)}))
  }
}



pub trait Group : View {
  fn get_name(&self) -> String;
}

//View Implemantation by delegation
impl View for TGroup {
  fn get_owner(&self) -> OWGroupLink {
    self.view.get_owner()
  }
  
  fn set_bounds(&mut self, bounds: TRect) {
    self.view.set_bounds(bounds)
  }

  fn get_bounds(&self) -> TRect {
    self.view.get_bounds()
  }

  fn get_extent(&self) -> TRect {
    self.view.get_extent()
  }
}

impl Group for TGroup {
  fn get_name(&self) -> String {
    self.name.clone()
  }

}
