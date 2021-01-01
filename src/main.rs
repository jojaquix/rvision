
extern crate rvision;
use crate::rvision::screen;
use crate::rvision::point;


fn main() {
    println!("Hello, world!");
    screen::say_hi_base();
    screen::say_hi();
    let _res = screen::TResolution  { x:80, y:25};
    screen::clear();
    screen::set_cursor_pos(20, 10);
    screen::say_hi();

    let cur_pos = screen::get_cursor_pos();

    println!("{:?}", cur_pos);

    let mut _p1 = point::TPoint { x: 10, y: 20};
   
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
    fn get_cursor_pos_test() {
        screen::set_cursor_pos(0, 0);
        let pos = screen::get_cursor_pos();
        println!("{:?}", pos);
    }

    #[test]
    fn point_test() {
        let p1 = point::TPoint { x: 10, y: 20};
        let p2 = point::TPoint { x: 5, y: 5};
        let p3 = p1 + p2;
        assert!(p3.x == 15 && p3.y == 25);
        assert!(!point::TPoint::equal(&p1, &p2));
        let p4 = point::TPoint::add(&p1, &p2);
        assert!(point::TPoint::equal(&p4, &p3));
        let p5 = p1 - p2;
        assert!(p5.x == 5 && p5.y == 15);
    }

    



}
