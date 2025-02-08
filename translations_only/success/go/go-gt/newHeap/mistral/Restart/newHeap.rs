
use std::mem;

const LEN: usize = 6;

#[repr(C)]
struct Int6 {
    data: [i32; LEN],
}

fn create_int6() -> Box<Int6> {
    Box::new(Int6 {
        data: [0; LEN],
    })
}

fn get_data(int6: &Int6) -> &[i32] {
    &int6.data
}

fn set_data(int6: &mut Int6, data: &[i32]) {
    if data.len() != LEN {
        panic!("data slice must have length {}", LEN);
    }
    for i in 0..LEN {
        int6.data[i] = data[i];
    }
}

fn destroy_int6(int6: Box<Int6>) {
    mem::drop(int6);
}
