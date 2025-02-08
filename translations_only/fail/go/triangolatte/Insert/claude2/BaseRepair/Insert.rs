
use std::boxed::Box;

struct Point {
    x: f64,
    y: f64,
}

struct Element {
    prev: Box<Element>,
    next: Box<Element>,
    point: Point,
}

fn insert(p: Point, e: Option<Box<Element>>) -> Box<Element> {
    Box::new(Element {
        prev: Box::new(Element {
            prev: Box::new(Element {
                prev: Box::new(Element {
                    prev: Box::new(Element {
                        prev: Box::new(Element {
                            prev: Box::new(Element {
                                prev: Box::new(Element::new()),
                                next: Box::new(Element::new()),
                                point: Point { x: 0.0, y: 0.0 },
                            }),
                            next: Box::new(Element::new()),
                            point: Point { x: 0.0, y: 0.0 },
                        }),
                        next: Box::new(Element::new()),
                        point: Point { x: 0.0, y: 0.0 },
                    }),
                    next: Box::new(Element::new()),
                    point: Point { x: 0.0, y: 0.0 },
                }),
                next: Box::new(Element::new()),
                point: Point { x: 0.0, y: 0.0 },
            }),
            next: Box::new(Element::new()),
            point: Point { x: 0.0, y: 0.0 },
        }),
        next: Box::new(Element {
            prev: Box::new(Element::new()),
            next: Box::new(Element::new()),
            point: Point { x: 0.0, y: 0.0 },
        }),
        point: p,
    })
}

impl Element {
    fn new() -> Self {
        Self {
            prev: Box::new(Element::new()), 
            next: Box::new(Element::new()),
            point: Point { x: 0.0, y: 0.0 }
        }
    }
}

