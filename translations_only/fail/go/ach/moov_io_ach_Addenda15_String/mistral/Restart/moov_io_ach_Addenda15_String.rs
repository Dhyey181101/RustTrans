
use std::boxed::Box;

struct Foo {
    data: i32,
}

fn main() {
    let f = Box::new(Foo { data: 42 });
    println!("{}", f.data);
}
