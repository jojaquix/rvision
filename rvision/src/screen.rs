
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


static TDISPLAY_MODES_NUM: i32 = 18;





pub fn say_hi_base() {
 println!("hello base");
}


#[cfg_attr(target_os = "linux", path = "linux/screen.rs")]
#[cfg_attr(target_os = "windows", path = "windows/screen.rs")]
mod imp;

pub fn say_hi() {
  imp::say_hi();
}









