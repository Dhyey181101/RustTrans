
use std::mem;

const LATENCY_SAMPLES: usize = 90;
const FILTER_TAPS: usize = 16;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const NB_CHANNELS: usize = 2;

#[derive(Copy, Clone)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Copy, Clone)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[Box<AptxFilterSignal>; NB_FILTERS]; NB_FILTERS],
}

struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],
    qmf: AptxQMFAnalysis,
    quantize: Box<[AptxQuantize; NB_SUBBANDS]>,
    invert_quantize: Box<[AptxInvertQuantize; NB_SUBBANDS]>,
    prediction: Box<[AptxPrediction; NB_SUBBANDS]>,
}

struct AptxContext {
    decode_sync_packets: usize,
    decode_dropped: usize,
    channels: [Box<AptxChannel>; NB_CHANNELS],
    hd: u8,
    sync_idx: u8,
    encode_remaining: u8,
    decode_skip_leading: u8,
    decode_sync_buffer_len: u8,
    decode_sync_buffer: [u8; 6],
}

fn aptx_reset(ctx: &mut AptxContext) {
    let hd = ctx.hd;
    let mut i: usize = 0;
    let mut chan: usize;
    let mut subband: usize;
    let mut channel: &mut Box<AptxChannel>;
    let mut prediction: &mut AptxPrediction;

    while i < mem::size_of::<AptxContext>() {
        ctx.decode_sync_buffer[i] = 0;
        i = i.wrapping_add(1);
    }

    ctx.hd = hd;
    ctx.decode_skip_leading = ((LATENCY_SAMPLES + 3) / 4) as u8;
    ctx.encode_remaining = ((LATENCY_SAMPLES + 3) / 4) as u8;

    chan = 0;
    while chan < NB_CHANNELS {
        channel = &mut ctx.channels[chan];
        subband = 0;
        while subband < NB_SUBBANDS {
            prediction = &mut channel.prediction[subband];
            prediction.prev_sign[0] = 1;
            prediction.prev_sign[1] = 1;
            subband += 1;
        }
        chan += 1;
    }
}

fn aptx_init(hd: i32) -> Box<AptxContext> {
    let mut channels: [Box<AptxChannel>; NB_CHANNELS] = unsafe { mem::uninitialized() };

    for chan in 0..NB_CHANNELS {
        let mut inner_filter_signal: [[Box<AptxFilterSignal>; NB_FILTERS]; NB_FILTERS] =
            unsafe { mem::uninitialized() };
        for i in 0..NB_FILTERS {
            for j in 0..NB_FILTERS {
                inner_filter_signal[i][j] = Box::new(AptxFilterSignal {
                    buffer: [0; 2 * FILTER_TAPS],
                    pos: 0,
                });
            }
        }

        channels[chan] = Box::new(AptxChannel {
            codeword_history: 0,
            dither_parity: 0,
            dither: [0; NB_SUBBANDS],
            qmf: AptxQMFAnalysis {
                outer_filter_signal: [AptxFilterSignal {
                    buffer: [0; 2 * FILTER_TAPS],
                    pos: 0,
                }; NB_FILTERS],
                inner_filter_signal,
            },
            quantize: Box::new([AptxQuantize {
                quantized_sample: 0,
                quantized_sample_parity_change: 0,
                error: 0,
            }; NB_SUBBANDS]),
            invert_quantize: Box::new([AptxInvertQuantize {
                quantization_factor: 0,
                factor_select: 0,
                reconstructed_difference: 0,
            }; NB_SUBBANDS]),
            prediction: Box::new([AptxPrediction {
                prev_sign: [1, 1],
                s_weight: [0, 0],
                d_weight: [0; 24],
                pos: 0,
                reconstructed_differences: [0; 48],
                previous_reconstructed_sample: 0,
                predicted_difference: 0,
                predicted_sample: 0,
            }; NB_SUBBANDS]),
        });
    }

    let mut ctx = Box::new(AptxContext {
        decode_sync_packets: 0,
        decode_dropped: 0,
        channels,
        hd: if hd != 0 { 1 } else { 0 },
        sync_idx: 0,
        encode_remaining: 0,
        decode_skip_leading: 0,
        decode_sync_buffer_len: 0,
        decode_sync_buffer: [0; 6],
    });

    aptx_reset(&mut ctx);
    ctx
}
