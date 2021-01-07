extern crate rvision;
use crate::rvision::screen;
use crate::rvision::point;
use crate::rvision::rect;
use crate::rvision::view;



fn main() {
    //println!("Hello, world!");
    //screen::say_hi_base();
    //screen::say_hi();
    let _res = screen::TResolution  { x:80, y:25};
    screen::clear();
    //screen::set_cursor_pos(0, 10);
    let mut cur_pos = screen::get_cursor_pos();
    //println!("{:?}", cur_pos);
    //screen::say_hi();
    cur_pos = screen::get_cursor_pos();
    //println!("{:?}", cur_pos);

    //let mut _p1 = point::TPoint { x: cur_pos.0 as i16, y: cur_pos.1 as i16};
    let mut r1 = rect::TRect { 
                a: point::TPoint {x: 5, y: 5}, 
                b: point::TPoint{x: 20, y: 10 }
            };
    //print!("{}",3*'c'); 

    let tview = view::TView::new(r1);
    tview.write_char(1, 1, '=', 0, 20);

    
    let r2 = rect::TRect { 
        a: point::TPoint {x: 50, y: 10}, 
        b: point::TPoint{x: 60, y: 20 }
    };

    let tview2 = view::TView::new(r2);
    tview2.write_line(0, 0, 11, 11,  '#');
    tview2.write_line(1, 1, 8, 8,  '$');



}

#[cfg(test)]
mod test {
    use super::*;    
    use crate::point::*;
    use crate::rect::*;

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
        screen::set_cursor_pos(0, 10);
        let pos = screen::get_cursor_pos();
        println!("{:?}", pos);
    }

    #[test]
    fn point_test() {
        let p1 = TPoint { x: 10, y: 20};
        let p2 = point::TPoint { x: 5, y: 5};
        let p3 = p1 + p2;
        assert!(p3.x == 15 && p3.y == 25);
        assert!(!point::TPoint::equal(p1, p2));
        let p4 = point::TPoint::add(p1, p2);
        assert!(point::TPoint::equal(p4, p3));
        let p5 = p1 - p2;
        assert!(p5.x == 5 && p5.y == 15);

        let mut p6 = p1;
        p6 += p2;
        assert!(point::TPoint::equal(p6, p3));

        let mut p7 = p6;
        p7 -= p2;
        assert!(point::TPoint::equal(p7, p1));

    }

    #[test]
    fn rect_test() {
        let r1 = TRect { 
            a: TPoint {x: 10, y: 10}, 
            b: TPoint{x: 40, y: 40 }
        };

        let r2 = TRect { 
            a: TPoint {x: 20, y: 20}, 
            b: TPoint{x: 50, y: 50 }
        };

        let p1 = TPoint { x: 5, y: 5};
        let p2 = TPoint { x: 15, y: 15};

        assert!(r1.contains(p2));
        assert!(!r1.contains(p1));

        assert!(!r2.contains(p2));
        assert!(!r2.contains(p1));

        assert!(!r1.equalr(&r2));
        assert!(!r2.equalr(&r1));

        assert!(!r1.equal(r2));
        assert!(!r2.equal(r1));

        let mut r11 = r1;
        r11.intersect(r2);
        assert!(r11.equal(TRect { a: TPoint {x: 20, y: 20}, 
            b: TPoint{x: 40, y: 40 }}));

        assert!(!r11.equal(r1));

        let mut r12 = r1;
        r12.union(r2);
        assert!(r12.equal(TRect { a: TPoint {x: 10, y: 10}, 
            b: TPoint{x: 50, y: 50 }}));

    }

}
