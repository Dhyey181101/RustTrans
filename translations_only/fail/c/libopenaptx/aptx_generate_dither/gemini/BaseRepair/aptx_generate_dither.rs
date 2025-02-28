

use std::ops::{Add, Shl, Shr};

const FILTER_TAPS: usize = 10;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 3;

#[derive(Debug, Clone, Copy)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Debug, Clone, Copy)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Debug, Clone, Copy)]
struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[derive(Debug, Clone, Copy)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],

    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

impl AptxChannel {
    fn update_codeword_history(&mut self) {
        let cw = ((self.quantize[0].quantized_sample & 3) << 0)
            + ((self.quantize[1].quantized_sample & 2) << 1)
            + ((self.quantize[2].quantized_sample & 1) << 3);
        self.codeword_history = (cw << 8) + (self.codeword_history >> 4);
    }

    fn generate_dither(&mut self) {
        let mut m: i64 = (5184443 * (self.codeword_history >> 7)) as i64;
        let d = ((m * 4) + (m >> 22)) as i32;
        for subband in 0..NB_SUBBANDS {
            self.dither[subband] = (d as i32) << (23 - 5 * subband);
        }
        self.dither_parity = (d >> 25) & 1;
    }
}


