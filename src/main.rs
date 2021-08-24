extern crate rvision;
use crate::rvision::drawbuf;
use crate::rvision::group::*;
use crate::rvision::point;
use crate::rvision::rect;
use crate::rvision::screen;
use crate::rvision::view;
use crate::rvision::view::*;

use std::rc::Rc;

fn main() {
    //println!("Hello, world!");
    //screen::say_hi_base();
    //screen::say_hi();
    screen::init();
    let _res = screen::TResolution { x: 80, y: 25 };
    screen::clear();
    //screen::set_cursor_pos(0, 10);
    let mut cur_pos = screen::get_cursor_pos();
    //println!("{:?}", cur_pos);
    //screen::say_hi();
    cur_pos = screen::get_cursor_pos();
    //println!("{:?}", cur_pos);

    //let mut _p1 = point::TPoint { x: cur_pos.0 as i16, y: cur_pos.1 as i16};
    let  r = rect::TRect {
        a: point::TPoint { x: 1,  y: 1 },
        b: point::TPoint { x: 80, y: 30 },
    };    

    let tgroup = TGroup::new(String::from("Jhon"), r, None);

    let  r1 = rect::TRect {
        a: point::TPoint { x: 1, y: 1 },
        b: point::TPoint { x: 40, y: 15 },
    };
    let tview = view::TView::new(r1, Some(Rc::downgrade(&tgroup)));
    tview.draw();
    
    tview.write_char(2, 1, 'y', 10, 2);
    

    let r2 = rect::TRect {
        a: point::TPoint { x: 21, y: 1 },
        b: point::TPoint { x: 45, y: 15 },
    };

    //let tview2 = view::TView::new(r2, None);
    let tview2 = view::TView::new(r2, Some(Rc::downgrade(&tgroup)));
    tview2.draw();
    //tview2.write_line(1, 1, 8, 1, 'h');
    //tview2.write_line(1, 2, 8, 8, 'a');

    screen::set_cursor_pos(0, 0);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::drawbuf::*;
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
        let p1 = TPoint { x: 10, y: 20 };
        let p2 = point::TPoint { x: 5, y: 5 };
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
            a: TPoint { x: 10, y: 10 },
            b: TPoint { x: 40, y: 40 },
        };

        let r2 = TRect {
            a: TPoint { x: 20, y: 20 },
            b: TPoint { x: 50, y: 50 },
        };

        let p1 = TPoint { x: 5, y: 5 };
        let p2 = TPoint { x: 15, y: 15 };

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
        assert!(r11.equal(TRect {
            a: TPoint { x: 20, y: 20 },
            b: TPoint { x: 40, y: 40 }
        }));

        assert!(!r11.equal(r1));

        let mut r12 = r1;
        r12.union(r2);
        assert!(r12.equal(TRect {
            a: TPoint { x: 10, y: 10 },
            b: TPoint { x: 50, y: 50 }
        }));
    }

    #[test]
    fn tdrawbuff_test() {
        let mut df1 = TDrawBuffer::new();
        df1.move_char(2, 'न', 3, 10);

        assert!(df1.get_char_at(0) == ' ');
        assert!(df1.get_char_at(1) == ' ');
        assert!(df1.get_char_at(2) == 'न');
        assert!(df1.get_attr_at(1) == 0);
        assert!(df1.get_attr_at(2) == 3);
        
    }

    #[test]
    fn tdrawbuff_test2() {
        let mut df = TDrawBuffer::new();
        df.move_string(2, String::from("태권도"), 3);
        
    }

}
