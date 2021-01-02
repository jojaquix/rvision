use crate::point::*;
use std::cmp;

#[derive(Copy, Clone)]
pub struct TRect {
  pub a : TPoint,
  pub b: TPoint
}


impl TRect {

  pub fn move_delta(&mut self, dx: i16, dy: i16) {
      self.a.x += dx;
      self.a.y += dy;
      self.b.x += dx;
      self.b.y += dy;
  }

  pub fn grow_delta(&mut self, dx: i16, dy: i16) {
    self.a.x -= dx;
    self.a.y -= dy;
    self.b.x += dx;
    self.b.y += dy;
  }

  pub fn intersect(&mut self, r: TRect) {
    self.a.x = cmp::max( self.a.x, r.a.x );
    self.a.y = cmp::max( self.a.y, r.a.y );
    self.b.x = cmp::min( self.b.x, r.b.x );
    self.b.y = cmp::min( self.b.y, r.b.y );
  }

  pub fn union(&mut self, r: TRect) {
    self.a.x = cmp::min( self.a.x, r.a.x );
    self.a.y = cmp::min( self.a.y, r.a.y );
    self.b.x = cmp::max( self.b.x, r.b.x );
    self.b.y = cmp::max( self.b.y, r.b.y );
  }

  pub fn contains(&self, p: TPoint) -> bool {
    p.x >= self.a.x && p.x < self.b.x && p.y >= self.a.y && p.y < self.b.y 
  }

  pub fn equal(&self, r: TRect) -> bool {
    TPoint::equalr(&self.a, &r.a) && TPoint::equalr(&self.b, &r.b)
  }

  pub fn equalr(&self, r: &TRect) -> bool {
    TPoint::equalr(&self.a, &r.a) && TPoint::equalr(&self.b, &r.b)
  }

  pub fn is_empty(&self) -> bool {
    self.a.x >= self.b.x || self.a.y >= self.b.y
  }

}

