
use std::boxed::Box;

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 24;

#[derive(Clone)]
struct AptxFilterSignal {
    buffer: [i32; 2 * NB_FILTERS],
    pos: u8,
}

#[derive(Clone)]
struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; NB_SUBBANDS],
    pos: i32,
    reconstructed_differences: [i32; 48],
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

#[derive(Clone)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Clone)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Clone)]
struct AptxQMFAnalysis {
    outer_filter_signal: [Box<AptxFilterSignal>; NB_FILTERS],
    inner_filter_signal: [[Box<AptxFilterSignal>; NB_FILTERS]; NB_FILTERS],
}

#[derive(Clone)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],
    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
    }
    parity & 1
}
