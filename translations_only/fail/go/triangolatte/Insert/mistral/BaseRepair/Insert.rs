
use std::boxed::Box;
use std::rc::Rc;
use std::cell::RefCell;

type Point = (f64, f64);

// Function to create a boxed Point
fn boxed_point(x: f64, y: f64) -> Box<Point> {
    Box::new((x, y))
}

// Function to create a reference counted Point
fn rc_point(x: f64, y: f64) -> Rc<RefCell<Point>> {
    Rc::new(RefCell::new((x, y)))
}
