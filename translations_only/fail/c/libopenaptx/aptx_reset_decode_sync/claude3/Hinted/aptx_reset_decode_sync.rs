
use std::mem;

const LATENCY_SAMPLES: usize = 90;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

enum Channels {
    Left,
    Right,
    NbChannels,
}

struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: u8,
}

struct AptxPrediction {
    prev_sign: Box<[i32; 2]>,
    s_weight: Box<[i32; 2]>,
    d_weight: Box<[i32; 24]>,
    pos: i32,
    reconstructed_differences: Box<[i32; 48]>,
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

struct AptxQMFAnalysis {
    outer_filter_signal: Box<[AptxFilterSignal; NB_FILTERS]>,
    inner_filter_signal: Box<[[AptxFilterSignal; NB_FILTERS]; NB_FILTERS]>,
}

struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: Box<[i32; NB_SUBBANDS]>,
    qmf: AptxQMFAnalysis,
    quantize: Box<[AptxQuantize; NB_SUBBANDS]>,
    invert_quantize: Box<[AptxInvertQuantize; NB_SUBBANDS]>,
    prediction: Box<[AptxPrediction; NB_SUBBANDS]>,
}

struct AptxContext {
    decode_sync_packets: usize,
    decode_dropped: usize,
    channels: Box<[AptxChannel; 2]>,
    hd: u8,
    sync_idx: u8,
    encode_remaining: u8,
    decode_skip_leading: u8,
    decode_sync_buffer_len: u8,
    decode_sync_buffer: Box<[u8; 6]>,
}

fn aptx_reset(ctx: &mut AptxContext) {
    let hd = ctx.hd;
    let mut i: usize = 0;
    let mut chan: usize;
    let mut subband: usize;
    let mut channel: &mut AptxChannel;
    let mut prediction: &mut AptxPrediction;

    while i < mem::size_of::<AptxContext>() {
        unsafe {
            *((ctx as *mut AptxContext).offset(i as isize) as *mut u8) = 0;
        }
        i += 1;
    }

    ctx.hd = hd;
    ctx.decode_skip_leading = ((LATENCY_SAMPLES + 3) / 4) as u8;
    ctx.encode_remaining = ((LATENCY_SAMPLES + 3) / 4) as u8;

    for chan in 0..Channels::NbChannels as usize {
        channel = &mut ctx.channels[chan];
        for subband in 0..NB_SUBBANDS {
            prediction = &mut channel.prediction[subband];
            prediction.prev_sign[0] = 1;
            prediction.prev_sign[1] = 1;
        }
    }
}

fn aptx_reset_decode_sync(ctx: &mut AptxContext) {
    let decode_dropped = ctx.decode_dropped;
    let decode_sync_packets = ctx.decode_sync_packets;
    let decode_sync_buffer_len = ctx.decode_sync_buffer_len;
    let mut decode_sync_buffer: Box<[u8; 6]> = Box::new([0; 6]);
    let mut i: usize = 0;

    while i < 6 {
        decode_sync_buffer[i] = ctx.decode_sync_buffer[i];
        i += 1;
    }

    aptx_reset(ctx);

    i = 0;
    while i < 6 {
        ctx.decode_sync_buffer[i] = decode_sync_buffer[i];
        i += 1;
    }

    ctx.decode_sync_buffer_len = decode_sync_buffer_len;
    ctx.decode_sync_packets = decode_sync_packets;
    ctx.decode_dropped = decode_dropped;
}
