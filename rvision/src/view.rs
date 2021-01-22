use crate::point::*;
use crate::rect::*;
use crate::screen::*;
use crate::drawbuf::*;
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
  pub fn new(bounds: TRect) -> Self {
    let tview = TView { origin: bounds.a, size: bounds.b - bounds.a};
    tview
  }
  
/*  pub fn set_bounds(&mut self, bounds: TRect) {
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
  
  pub fn write_char(&self, x :i16, y: i16, c: char, color: u16, count: i16) {
    let bounds = self.get_bounds();
    //todo use make_global for now tviews does not have group
    set_color(color);
    write_nchar((bounds.a.x + x) as u16, (bounds.a.y + y) as u16, c, count);
  }

  ///
  //todo change to paint string or buffer as in TV
  pub fn write_line(&self, x :i16, y: i16, w:i16, h: i16,  c: char) {
    let bounds = self.get_bounds();
    let w2 = cmp::min(self.get_extent().b.x, w);
    let h2 = cmp::min(self.get_extent().b.y, h);
    for i in 0..h2 {
      write_nchar((bounds.a.x + x) as u16, (bounds.a.y + y + i) as u16, c, w2);
    }
  }
  
  
  pub fn write_line_buffer (&self, x :i16, y: i16, w:i16, h: i16,  buf: TDrawBuffer) {
    let bounds = self.get_bounds();
    let w2 = cmp::min(self.get_extent().b.x, w);
    let h2 = cmp::min(self.get_extent().b.y, h);
    for i in 0..h2 {
      write_nchar((bounds.a.x + x) as u16, (bounds.a.y + y + i) as u16, '$', w2);
    }    
  }*/
}

// common functionality that needs some T with View trait
//trait Displayable {}

//impl<T> Displayable for T where T : View {}

//todo add default imps based on others methods of trait

pub trait View {

  // Returns the current value of size, the bounding rectangle of the view
  // in its owner's coordinate system.
  fn get_bounds(&self) -> TRect;

  // Returns the extent rectangle of the view.
  fn get_extent(&self) -> TRect;
  
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

  fn set_bounds(&mut self, bounds: TRect);


  // the most important method for any displayable object
  fn draw(&self) {
    let bounds = self.get_bounds();
    //todo make global ?
    write_char(bounds.a.x as u16, bounds.a.y as u16, '╔');
    write_char(bounds.b.x as u16, bounds.a.y as u16, '╗');
    write_char(bounds.a.x as u16, bounds.b.y as u16, '╚');
    write_char(bounds.b.x as u16, bounds.b.y as u16, '╝');
  }

  //fn draw_if_exposed(&self);


  // write functions
   
  /// Beginning at the point (x, y), writes 'count' copies of the character
  /// 'c' in the color determined by the color'th entry in the current view's
  /// palette. Should only be used in @ref draw() functions.
  /// I guess the behaivior is: 
  /// If x + count exceds limits start next posible coodinate one line below ????
  /// 
  
  fn write_char(&self, x :i16, y: i16, c: char, color: u16, count: i16) {
    //from original writechar
    let mut nx = if x<0 { 0i16 } else { x };
    if nx+count > MAX_VIEW_WIDTH as i16 { return };
    
    //from writeview
    let bounds = self.get_bounds();
    if y<0 || y > bounds.b.y { return };
    //let mut nx = if x<0 { 0i16 } else { x };
    let mut x2 = if nx + count > bounds.b.x { bounds.b.x } else { nx + count };
    if nx > x2 { return };  

    //todo use make_global for now tviews does not have group
    set_color(color);    
    //todo change this for something like writeview
    //for now ! this !! count must not change
    //calcs are in nx and x2 and passed to writeview 
    let ncount = range(count, 0, self.get_extent().b.x - nx + 1); //this will change!!!!!
    write_nchar((bounds.a.x + nx) as u16, (bounds.a.y + y) as u16, c, ncount);
    //print!("nx {}", nx);
  }
  
  /**
   * Writes the line contained in the buffer 'b' to the screen, beginning at
   * the point (x, y) within the rectangle defined by the width 'w' and the
   * height 'h'. If 'h' is greater than 1, the line will be repeated 'h'
   * times. Should only be used in @ref draw() member functions.
   * TODO: change c by string b
   */  
  
  fn write_line(&self, x :i16, y: i16, w:i16, h: i16,  c: char) {
    let bounds = self.get_bounds();
    let w2 = cmp::min(self.get_extent().b.x, w);
    let h2 = cmp::min(self.get_extent().b.y, h);
    for i in 0..h2 {
      write_nchar((bounds.a.x + x) as u16, (bounds.a.y + y + i) as u16, c, w2);
    }
  }
  
  /**
   * Writes the line contained in the buffer `b' to the screen, beginning at
   * the point (x, y) within the rectangle defined by the width `w' and the
   * height `h'. If `h' is greater than 1, the line will be repeated `h'
   * times. Should only be used in @ref draw() member functions.
   * @see TDrawBuffer
   */    
  fn write_line_buffer (&self, x :i16, y: i16, w:i16, h: i16,  buf: TDrawBuffer) {
    let bounds = self.get_bounds();
    let w2 = cmp::min(self.get_extent().b.x, w);
    let h2 = cmp::min(self.get_extent().b.y, h);
    for i in 0..h2 {
      write_nchar((bounds.a.x + x) as u16, (bounds.a.y + y + i) as u16, '$', w2);
    }
  }
}

  impl View for TView {

  fn set_bounds(&mut self, bounds: TRect) {
    self.origin = bounds.a;
    self.size = bounds.b - bounds.a;
  }

  fn get_bounds(&self) -> TRect {
    TRect { a: self.origin, b: self.origin + self.size }
  }

  fn get_extent(&self) -> TRect {
    TRect { a: TPoint {x: 0, y: 0}, b: self.size }
  }

}