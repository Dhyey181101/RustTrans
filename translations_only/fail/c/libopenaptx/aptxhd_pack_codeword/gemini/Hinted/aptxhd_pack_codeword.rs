

use std::mem;

#[derive(Clone, Copy)]
struct AptxFilterSignal {
    buffer: [i32; 2 * 128],
    pos: u8,
}

#[derive(Clone, Copy)]
struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: [i32; 48],
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

#[derive(Clone, Copy)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Clone, Copy)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Clone, Copy)]
struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; 10],
    inner_filter_signal: [[AptxFilterSignal; 10]; 10],
}

#[derive(Clone, Copy)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; 5],

    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; 5],
    invert_quantize: [AptxInvertQuantize; 5],
    prediction: [AptxPrediction; 5],
}

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..5 {
        parity ^= channel.quantize[subband].quantized_sample;
    }
    parity & 1
}

fn aptxhd_pack_codeword(channel: &AptxChannel) -> u32 {
    let parity = aptx_quantized_parity(channel);
    ((channel.quantize[3].quantized_sample as u32 & 0x01E) | (parity as u32)) << 19
        | ((channel.quantize[2].quantized_sample as u32 & 0x00F) << 15)
        | ((channel.quantize[1].quantized_sample as u32 & 0x03F) << 9)
        | ((channel.quantize[0].quantized_sample as u32 & 0x1FF) << 0)
}

