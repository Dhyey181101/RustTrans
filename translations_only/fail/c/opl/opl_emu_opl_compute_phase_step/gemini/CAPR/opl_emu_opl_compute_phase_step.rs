
use std::ops::{Shl, Shr};

pub fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let mask = ((1 << length) - 1) << start;
    (value & mask) >> start
}

pub fn opl_emu_opl_compute_phase_step(
    block_freq: u32,
    multiple: u32,
    lfo_raw_pm: i32,
) -> u32 {
    // extract frequency number as a 12-bit fraction
    let fnum = opl_emu_bitfield(block_freq, 0, 10) << 2;

    // apply the phase adjustment based on the upper 3 bits
    // of FNUM and the PM depth parameters
    let fnum = fnum.wrapping_add((lfo_raw_pm as u32 * opl_emu_bitfield(block_freq, 7, 3)) >> 1);

    // keep fnum to 12 bits
    let fnum = fnum & 0xfff;

    // apply block shift to compute phase step
    let block = opl_emu_bitfield(block_freq, 10, 3);
    let phase_step = (fnum << block) >> 2;

    // apply frequency multiplier (which is cached as an x.1 value)
    (phase_step * multiple) >> 1
}
