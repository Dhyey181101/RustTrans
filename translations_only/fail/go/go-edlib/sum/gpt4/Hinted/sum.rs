
fn sum(arr: &[isize]) -> isize {
    let mut res = 0;
    for &v in arr {
        res += v;
    }
    res
}

fn main() {}
