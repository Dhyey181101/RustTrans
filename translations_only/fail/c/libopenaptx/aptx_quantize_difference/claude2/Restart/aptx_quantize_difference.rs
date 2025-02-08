

use std::convert::TryInto;

struct AptxTables {
    quantize_intervals: Box<[i32]>,
    invert_quantize_dither_factors: Box<[i32]>,
    quantize_dither_factors: Box<[i32]>,
    quantize_factor_select_offset: Box<[i16]>,
    tables_size: i32,
    factor_max: i32,
    prediction_order: i32,
}

struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1 << p) & !((2 << p) - 1)) > i32::MAX as u32 {
        (!(a >> 31)) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - (((value & mask) == rounding) as i64)
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift).try_into().unwrap(), 23)
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32 << (shift - 1);
    let mask = (1i32 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - (((value & mask) == rounding) as i32)
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}

fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: i32) -> i32 {
    let mut idx = 0;
    let mut i = nb_intervals >> 1;

    while i > 0 {
        if (factor as i64) * (intervals[idx as usize + (i as usize)] as i64) <= ((value as i64) << 24) {
            idx += i;
        }
        i >>= 1;
    }

    idx
}

fn aptx_quantize_difference(quantize: &mut AptxQuantize, sample_difference: i32, dither: i32,
    quantization_factor: i32, tables: &AptxTables) {

    // Rest of function
}


