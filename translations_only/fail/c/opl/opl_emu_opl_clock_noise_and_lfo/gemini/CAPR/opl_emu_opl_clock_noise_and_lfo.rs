
use std::ops::{Shl, Shr};

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_opl_clock_noise_and_lfo(
    noise_lfsr: &mut u32,
    lfo_am_counter: &mut u16,
    lfo_pm_counter: &mut u16,
    lfo_am: &mut u8,
    am_depth: u32,
    pm_depth: u32,
) -> i32 {
    // OPL has a 23-bit noise generator for the rhythm section, running at
    // a constant rate, used only for percussion input
    *noise_lfsr = (*noise_lfsr).wrapping_shl(1);
    *noise_lfsr |= opl_emu_bitfield(*noise_lfsr, 23, 1)
        ^ opl_emu_bitfield(*noise_lfsr, 9, 1)
        ^ opl_emu_bitfield(*noise_lfsr, 8, 1)
        ^ opl_emu_bitfield(*noise_lfsr, 1, 1);

    // OPL has two fixed-frequency LFOs, one for AM, one for PM

    // the AM LFO has 210*64 steps; at a nominal 50kHz output,
    // this equates to a period of 50000/(210*64) = 3.72Hz
    let am_counter = *lfo_am_counter;
    *lfo_am_counter = if am_counter >= 210 * 64 - 1 { 0 } else { am_counter + 1 };

    // low 8 bits are fractional; depth 0 is divided by 2, while depth 1 is times 2
    let shift = 9 - 2 * am_depth;

    // AM value is the upper bits of the value, inverted across the midpoint
    // to produce a triangle
    *lfo_am = ((am_counter < 105 * 64) as u8)
        .wrapping_shr(shift)
        ^ ((210 * 64 + 63 - am_counter) as u8).wrapping_shr(shift);

    // the PM LFO has 8192 steps, or a nominal period of 6.1Hz
    let pm_counter = *lfo_pm_counter;
    *lfo_pm_counter = if pm_counter >= 8192 - 1 { 0 } else { pm_counter + 1 };

    // PM LFO is broken into 8 chunks, each lasting 1024 steps; the PM value
    // depends on the upper bits of FNUM, so this value is a fraction and
    // sign to apply to that value, as a 1.3 value
    let pm_scale = [8, 4, 0, -4, -8, -4, 0, 4];
    let pm_scale_index = opl_emu_bitfield(pm_counter as u32, 10, 3) as usize;
    let pm_scale_value = pm_scale[pm_scale_index];
    (pm_scale_value >> (pm_depth ^ 1)) as i32
}
