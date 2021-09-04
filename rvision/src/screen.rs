
pub struct TResolution {
    pub x: u16, 
    pub y: u16
}

pub struct TColor {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub alpha: u8
}

pub struct TFont256
{
 pub w: u32,
 pub h: u32,
 pub data: Vec<u8>
}


type TFontRequestCallBack = fn(which: i32, width: u16, height: u16 ) -> Box<TFont256>;
type TScreenDriverDetectedCallBack = fn();


const TDISPLAY_MODES_NUM: i32 = 18;


pub enum VideoModes
 {
  SmBw80    = 0x0002,
  SmCo80    = 0x0003, 
  SmMono    = 0x0007,
  SmFont8x8 = 0x0100 
 }

pub fn say_hi_base() {
 println!("hello base");
}


#[cfg_attr(target_os = "linux", path = "linux/screen.rs")]
#[cfg_attr(target_os = "windows", path = "windows/screen.rs")]
mod imp;

pub fn say_hi() {
  imp::say_hi()
}


/// init the screen
pub fn init() {
  imp::init()
}

/// clear entire the screen
pub fn clear() {
  imp::clear()
}

/// rows
pub fn get_rows() -> u16 {
  imp::get_rows()
}

/// cols
pub fn get_cols() -> u16 {  
  imp::get_cols()
}

pub fn get_resolution() -> TResolution {
  TResolution {x: get_rows(), y: get_cols()}
}

/// set cursor position
pub fn set_cursor_pos(x: u16, y: u16 ) {
  imp::set_cursor_pos(x, y)
}

/// get the current cursor position
/// (x, y) == (col, row)
pub fn get_cursor_pos() -> (u16, u16) {
  imp::get_cursor_pos()
}

pub fn set_color(color: u16) {
  imp::set_color(color);
}

/// write on screen global coordinates
pub fn write_char(x: u16, y: u16, c: char) {
  imp::write_char(x ,y , c);
}

pub fn write_nchar(x: u16, y: u16, c: char, count: i16) {
  imp::write_nchar(x ,y , c, count);
}

pub fn write_string(x: u16, y: u16, s: String) {
  imp::write_string(x ,y , s);
}





