
fn get(n: i64, a: Box<[i64]>, i: i64, j: i64) -> i64 {
    if n <= 0 || i < 0 || j < 0 || i >= n || j >= n {
        panic!("Input is invalid, crash gracefully");
    }
    a[i as usize * n as usize + j as usize]
}

#[test]
fn test_cases() {
    assert_eq!(get(0, Box::new([]), 0, 36442797506560), panic!("Input is invalid, crash gracefully"));
    assert_eq!(get(42949935105, Box::new([]), -281474976710656, -256), panic!("Input is invalid, crash gracefully"));
    assert_eq!(get(-1, Box::new([-201210627882955, -74, 281474963079167, -280379760050176]), 255, 255), -201210627882955);
    assert_eq!(get(-3221225473, Box::new([2533274790395904, -281474976710656, -1095216660481, 262144]), 0, 0), 2533274790395904);
}
