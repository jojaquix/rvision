use crate::point::*;
use crate::rect::*;
use crate::screen::*;
use std::cmp;

pub fn range<T: PartialOrd>(Val: T, Min: T , Max:T ) -> T {
  if Val < Min { Min } else if Val > Max  { Max } else  { Val } 
} 

/// As in the original tvision tview is
/// any object displayable in a rectangular portion of 
/// screen. 
/// from turbo vision for C++: 
/// "The displayable classes derived from TObject
/// give you object known as views."

pub struct TView {
  /// origin point on owner's coordinate system
  origin: TPoint,
  size: TPoint
}


impl TView {
  pub fn new(bounds: TRect) -> TView {
    let tview = TView { origin: bounds.a, size: bounds.b - bounds.a};
    tview
  }
  
  pub fn set_bounds(&mut self, bounds: TRect) {
    self.origin = bounds.a;
    self.size = bounds.b - bounds.a;
  }

  pub fn get_bounds(&self) -> TRect {
    TRect { a: self.origin, b: self.origin + self.size }
  }

  pub fn get_extent(&self) -> TRect {
    TRect { a: TPoint {x: 0, y: 0}, b: self.size }
  }


  /// Beginning at the point (x, y), writes 'count' copies of the character
  /// 'c' in the color determined by the color'th entry in the current view's
  /// palette. Should only be used in @ref draw() functions.
  
  pub fn write_char(&self, x :i16, y: i16, c: char, _color: u8, count: i16) {
    let bounds = self.get_bounds();
    //todo use make_global for now tviews does not have group
    //set_cursor_pos((bounds.a.x + x) as u16, (bounds.a.y + y) as u16);
    //use write_xx functions and create some functions using api
    //todo use color
    write_nchar((bounds.a.x + x) as u16, (bounds.a.y + y) as u16, c, count);
  }
  
  //todo change to paint string or buffer as in TV
  pub fn write_line(&self, x :i16, y: i16, w:i16, h: i16,  c: char) {
    let bounds = self.get_bounds();
    let w2 = cmp::min(self.get_extent().b.x, w);
    let h2 = cmp::min(self.get_extent().b.y, h);
    for i in 0..h2 {
      write_nchar((bounds.a.x + x) as u16, (bounds.a.y + y + i) as u16, c, w2);
    }
  } 
}

// common functionality that needs some T with View trait
//trait Displayable {}

//impl<T> Displayable for T where T : View {}

//todo add default imps based on others methods of trait

//trait View {

  // Returns the current value of size, the bounding rectangle of the view
  // in its owner's coordinate system.
 // fn get_bounds(&self) -> TRect

  // Returns the extent rectangle of the view.
 // fn get_extent(&self) -> TRect;
  
  // Changes the bounds of the view to those of the `bounds' argument.
  // The view is redrawn in its new location.
  // locate() calls @ref sizeLimits() to verify that the given bounds are
  // valid, and then calls @ref changeBounds() to change the bounds and
  // redraw the view.
  //fn locate(&mut self, bounds: TRect);

  // Moves the origin to the point (x,y) relative to the owner's view. The
  // view's size is unchanged.
  //fn move_to(&mut self, x :i16, y: i16);   

  // Returns in the ('min', 'max') , the minimum and maximum values
  // that @ref size data member may assume  
  //fn size_limits(&self) -> (TPoint, TPoint);

  // the most important method for any displayable object
  //fn draw(&self);

  //fn draw_if_exposed(&self);

  //write functions 
//}


//  impl View for TView {
//
//  fn get_bounds(&self) -> TRect {
//    TRect { a: self.origin, b: self.origin + self.size }
//  }
//
//  fn get_extent(&self) -> TRect {
//    TRect { a: TPoint {x: 0, y: 0}, b: self.size }
//  }
//
//}