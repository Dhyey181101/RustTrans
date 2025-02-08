
fn sum(arr: Vec<isize>) -> isize {
    let mut res: isize = 0;
    for v in arr {
        res += v;
    }
    res
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("{}", sum(numbers));
}
