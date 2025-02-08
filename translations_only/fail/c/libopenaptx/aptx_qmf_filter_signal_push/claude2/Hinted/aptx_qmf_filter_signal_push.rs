
use std::boxed::Box;
use std::mem;

const FILTER_TAPS: u8 = 16;

struct AptxFilterSignal {
    buffer: Box<[i32; 2*FILTER_TAPS as usize]>,
    pos: u8,
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos as usize] = sample;
    signal.buffer[(signal.pos + FILTER_TAPS) as usize] = sample;
    signal.pos = signal.pos.wrapping_add(1);
}
