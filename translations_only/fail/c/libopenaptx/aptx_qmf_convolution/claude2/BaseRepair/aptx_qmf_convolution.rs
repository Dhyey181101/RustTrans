
use std::convert::TryInto;

const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: Box<[i32; 2*FILTER_TAPS]>,
    pos: u8,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    let limit = (1u32 << p) - 1;
    if (a as u32).wrapping_add(1 << p) & !((2*limit as u32) - 1) != 0 {
        (!(a >> 31)) & (limit as i32)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1;
    ((value.wrapping_add(rounding)) >> shift) - (((value & mask) == rounding) as i64)   
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(value.try_into().unwrap(), 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, 
                        coeffs: &[i32; FILTER_TAPS],
                        shift: u32) -> i32 {
    let sig = &signal.buffer[signal.pos as usize..];
    let mut e: i64 = 0;
    for i in 0..FILTER_TAPS {
        e += sig[i] as i64 * coeffs[i] as i64;
    }
    rshift64_clip24(e, shift)
}
