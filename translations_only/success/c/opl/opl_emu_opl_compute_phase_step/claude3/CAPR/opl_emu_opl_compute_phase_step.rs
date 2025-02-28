

use std::num::Wrapping;

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let value = Wrapping(value);
    let shift = Wrapping(start as u32);
    let mask = Wrapping((1u32 << length) - 1);
    let shifted = value.0 >> shift.0;
    (shifted & mask.0)
}

fn opl_emu_opl_compute_phase_step(
    block_freq: u32,
    multiple: u32,
    lfo_raw_pm: i32,
) -> u32 {
    let fnum = opl_emu_bitfield(block_freq, 0, 10) << 2;
    let mut fnum = fnum
        .wrapping_add((((lfo_raw_pm * opl_emu_bitfield(block_freq, 7, 3) as i32) >> 1) as u32));
    fnum &= 0xfff;
    let block = opl_emu_bitfield(block_freq, 10, 3);
    let phase_step = (fnum << block) >> 2;
    (phase_step * multiple) >> 1
}

