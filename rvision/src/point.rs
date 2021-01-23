use std::ops::{Add, Sub, AddAssign, SubAssign};


/// One point on the screen, with some vector operations
#[derive(Copy, Clone, Debug)]
pub struct TPoint {
  pub x: i16,
  pub y: i16
}

impl TPoint {
  pub fn add(a: TPoint, b: TPoint) -> TPoint {
    TPoint {x : a.x + b.x, y: a.y + b.y }
  }

  pub fn sub(a: TPoint, b: TPoint) -> TPoint {
    TPoint {x : a.x - b.x, y: a.y - b.y }
  }

  pub fn equal (a: TPoint, b: TPoint) -> bool {
    a.x == b.x  &&  a.y == b.y
  }

  pub fn equalr (a: &TPoint, b: &TPoint) -> bool {
    a.x == b.x  &&  a.y == b.y
  }
  
}

impl Add for TPoint {
  type Output = Self;
  fn add(self, other: Self) -> Self {
      Self {x: self.x + other.x, y: self.y + other.y}
  }
}

impl Sub for TPoint {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
      Self {x: self.x - other.x, y: self.y - other.y}
  }
}

impl AddAssign for TPoint {
  fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
  }
}

impl SubAssign for TPoint {
  fn sub_assign(&mut self, other: Self) {
      *self = Self {
          x: self.x - other.x,
          y: self.y - other.y,
      };
  }
}
