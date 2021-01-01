
extern crate rvision;
use crate::rvision::screen;


fn main() {
    println!("Hello, world!");
    screen::say_hi_base();
    screen::say_hi();
    let _res = screen::TResolution  { x:80, y:25};
    screen::clear();
    screen::set_cursor_pos(20, 10);
    screen::say_hi();

    let cur_pos = screen::get_cursor_pos();

    println!("{:?}", cur_pos)
   
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn clear_test() {
        screen::clear();
    }

    #[test]
    fn get_rows_test() {
        assert!(screen::get_rows() > 0)
    }

    #[test]
    fn get_cols_test() {
        assert!(screen::get_cols() > 0)
    }

    #[test]
    fn set_cursor_pos_test() {
        screen::set_cursor_pos(10, 10)
    }

    #[test]
    fn get_cursor_pos_test() {
        screen::set_cursor_pos(0, 0);
        let pos = screen::get_cursor_pos();
        println!("{:?}", pos);
        assert!(pos.0 == 0 && pos.1 == 0);
    }

    



}
