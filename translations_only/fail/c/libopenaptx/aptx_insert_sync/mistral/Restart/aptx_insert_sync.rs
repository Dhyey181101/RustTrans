
use std::u8;

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

enum Channel {
    Left,
    Right,
    NbChannels,
}

#[repr(C)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

#[repr(C)]
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

#[repr(C)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[repr(C)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[repr(C)]
struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[repr(C)]
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

fn aptx_check_parity(channels: &[AptxChannel; 2], sync_idx: &mut u8) -> i32 {
    let parity = aptx_quantized_parity(&channels[0]) ^ aptx_quantized_parity(&channels[1]);
    let eighth = (*sync_idx == 7) as i32;
    *sync_idx = (*sync_idx + 1) & 7;
    parity ^ eighth
}

fn aptx_insert_sync(channels: &mut [AptxChannel; 2], sync_idx: &mut u8) {
    let mut min_error = i32::MAX;
    let mut min_index = 0;
    let mut c = &mut channels[1];
    let mut i = 0;
    let map = [1, 2, 0, 3];

    if aptx_check_parity(channels, sync_idx) == 0 {
        for c in &mut channels[..] {
            for subband in 0..NB_SUBBANDS {
                if c.quantize[map[subband]].error < min_error {
                    min_error = c.quantize[map[subband]].error;
                    min_index = subband;
                }
            }
        }

        c = &mut channels[min_index as usize / 2];
        c.quantize[map[min_index % 4]].quantized_sample =
            c.quantize[map[min_index % 4]].quantized_sample_parity_change;
    }
}
