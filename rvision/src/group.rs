use crate::point::*;
use crate::rect::*;
use crate::screen::*;
use crate::view::*;



pub struct TGroup {
  pub name: String,
}

pub trait Group { 
  
  fn get_name(&self) -> String;

  fn hello(&self) {
    print!("Hello from group trait {}", self.get_name())
  }
}

impl Group for TGroup {
  fn get_name(&self) -> String
  {
    self.name.clone()
  }
}






