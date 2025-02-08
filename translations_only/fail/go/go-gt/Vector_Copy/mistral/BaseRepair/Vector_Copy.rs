

use std::mem;
use std::boxed::Box;

fn main() {
 let boxed_i = Box::new(42);
 let mut boxed_f: Box<f64> = Box::new(1.0);
 let boxed_ptr = Box::into_raw(boxed_i) as *mut i32;

 let i = unsafe { Box::from_raw(boxed_ptr as *mut i32) };
 mem::drop(i);

 *boxed_f = 2.0;
}

