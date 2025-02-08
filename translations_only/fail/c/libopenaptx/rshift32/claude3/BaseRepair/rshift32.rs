
use std::i32;

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1 << (shift - 1);
    let mask = ((1 as i32) << (shift + 1)) - 1;
    let result = ((value + rounding) >> shift) - ((value & mask) == rounding) as i32;
    result
}
