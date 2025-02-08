
use std::boxed::Box;

fn sum(arr: Vec<isize>) -> isize {
    let mut res = 0;
    for v in arr {
        res += v;
    }
    res
}
