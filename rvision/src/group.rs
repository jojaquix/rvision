use crate::point::*;
use crate::rect::*;
use crate::screen::*;
use crate::view::*;



use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub type GroupLink = Rc<RefCell<dyn Group>>;
pub type WGroupLink = Weak<RefCell<dyn Group>>;
pub type OWGroupLink = Option<WGroupLink>;
pub type OGroupLink = Option<GroupLink>;

pub struct TGroup {
  pub this: OWGroupLink,
  view: TView,
  name: String,
  first: OViewLink,
  last: OViewLink,
}

impl TGroup {
  pub fn new(name: String, rect: TRect, owner: OWGroupLink) -> GroupLink {
    let mut group = Rc::new(RefCell::new(TGroup {
      name: name,
      view: TView::new(rect, owner),
      first: None,
      last: None,
      this: None,
    }));

    // would be necessary this 
    // group.borrow_mut().this = Some(Rc::downgrade(&group));
    group
  }
}

pub trait Group: View {
  fn get_name(&self) -> String;
  fn insert_view(&mut self, view: ViewLink, target: ViewLink);
}

//View Implemantation by delegation
impl View for TGroup {
  
  fn set_owner(&mut self, owner: OWGroupLink) {
    self.view.set_owner(owner)
  }  

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

  fn set_next(&mut self, next: OViewLink) { 
    self.view.set_next(next)
  }

  fn set_prev(&mut self, prev: OViewLink) { 
    self.view.set_prev(prev)
  } 

}

impl Group for TGroup {
  fn get_name(&self) -> String {
    self.name.clone()
  }

  //todo add the other
  fn insert_view(&mut self, view: ViewLink, target: ViewLink) {
    //view.borrow_mut().set_owner(Some(Rc::downgrade(&self)));

  }
}
