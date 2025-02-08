
use std::convert::TryInto;

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;

#[derive(Default)]
struct AptxFilterSignal {
    buffer: [i32; 2 * NB_FILTERS],
    pos: u8,
}

#[derive(Default)]
struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],  
    d_weight: Box<[i32; 24]>,
    pos: i32,
    reconstructed_differences: Box<[i32; 0]>,
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

#[derive(Default)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Default)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Default)]
struct AptxQmfAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[derive(Default)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],

    qmf: AptxQmfAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..NB_SUBBANDS {
        parity = parity.wrapping_add(channel.quantize[subband].quantized_sample);
    }
    (parity & 1) as i32
}

fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 32 - bits;
    ((val as i64).wrapping_shl(shift) as i32).wrapping_shr(shift)
}

fn aptx_unpack_codeword(channel: &mut AptxChannel, codeword: u16) {
    channel.quantize[0].quantized_sample = 
        sign_extend(codeword as i32 >> 0, 7);
    channel.quantize[1].quantized_sample = 
        sign_extend((codeword >> 7) as i32, 4);
    channel.quantize[2].quantized_sample = 
        sign_extend((codeword >> 11) as i32, 2);
    channel.quantize[3].quantized_sample = 
        sign_extend((codeword >> 13) as i32, 3);
    channel.quantize[3].quantized_sample &= !1;
    channel.quantize[3].quantized_sample |= aptx_quantized_parity(channel);
}

