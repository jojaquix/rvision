
extern crate rvision;
use crate::rvision::screen;



fn main() {
    println!("Hello, world!");
    screen::say_hi_base();
    screen::say_hi();
    let _res = screen::TResolution  { x:80, y:25};
    
}
