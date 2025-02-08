

use std::mem;

const QUANTIZATION_FACTORS: [i16; 32] = [
    2048, 2093, 2139, 2186, 2233, 2282, 2332, 2383,
    2435, 2489, 2543, 2599, 2656, 2714, 2774, 2834,
    2896, 2960, 3025, 3091, 3158, 3228, 3298, 3371,
    3444, 3520, 3597, 3676, 3756, 3838, 3922, 4008,
];

struct AptxTables {
    quantize_intervals: Vec<i32>,
    invert_quantize_dither_factors: Vec<i32>,
    quantize_dither_factors: Vec<i32>,
    quantize_factor_select_offset: Vec<i16>,
    tables_size: usize,
    factor_max: i32,
    prediction_order: i32,
}

struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

fn clip_intp2(a: u32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1 << p) & !(1 << (p + 1))).wrapping_sub(1) != 0 {
        return (a as i32 >> 31) ^ ((1 << p) - 1);
    } else {
        return a as i32;
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = (1 << (shift - 1)) as i64;
    let mask = ((1 << (shift + 1)) - 1) as i64;
    return ((value.wrapping_add(rounding)) >> shift).wrapping_sub((value & mask).wrapping_sub(rounding));
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    return clip_intp2(rshift64(value, shift) as u32, 23) as i32;
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = (1 << (shift - 1)) as i32;
    let mask = ((1 << (shift + 1)) - 1) as i32;
    return ((value.wrapping_add(rounding)) >> shift).wrapping_sub((value & mask).wrapping_sub(rounding));
}

fn clip(a: i32, amin: i32, amax: i32) -> i32 {
    if a < amin {
        return amin;
    } else if a > amax {
        return amax;
    } else {
        return a;
    }
}

fn aptx_invert_quantization(
    invert_quantize: &mut AptxInvertQuantize,
    quantized_sample: i32,
    dither: i32,
    tables: &AptxTables,
) {
    let idx = if quantized_sample < 0 {
        (quantized_sample.wrapping_neg() + 1) as usize
    } else {
        quantized_sample as usize + 1
    };
    let qr = tables.quantize_intervals[idx] / 2;
    let qr = if quantized_sample < 0 {
        -qr
    } else {
        qr
    };

    let qr = rshift64_clip24(
        ((qr as i64) * (1 << 32))
            .wrapping_add((dither as i64) * (tables.invert_quantize_dither_factors[idx] as i64)),
        32,
    );
    invert_quantize.reconstructed_difference = qr as i32;

    let mut factor_select = 32620 * invert_quantize.factor_select;
    factor_select = rshift32(
        factor_select
            .wrapping_add(
                (tables.quantize_factor_select_offset[idx] as i32)
                    .wrapping_mul(1 << 15),
            ),
        15,
    );
    invert_quantize.factor_select = clip(factor_select, 0, tables.factor_max);

    let idx = ((invert_quantize.factor_select & 0xFF) >> 3) as usize;
    let shift = ((tables.factor_max - invert_quantize.factor_select) >> 8) as u32;
    invert_quantize.quantization_factor =
        ((QUANTIZATION_FACTORS[idx] as i32) << 11) >> shift;
}

