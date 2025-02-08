

use libc::{uint32_t, uint16_t, uint8_t};
use std::boxed::Box;

fn opl_emu_bitfield(value: uint32_t, start: i32, length: i32) -> uint32_t {
    return (value >> start) & ((1 << length) - 1);
}

fn opl_emu_opl_clock_noise_and_lfo(
    noise_lfsr: &mut Box<uint32_t>,
    lfo_am_counter: &mut Box<uint16_t>,
    lfo_pm_counter: &mut Box<uint16_t>,
    lfo_am: &mut Box<uint8_t>,
    am_depth: uint32_t,
    pm_depth: uint32_t,
) -> i32 {
    // OPL has a 23-bit noise generator for the rhythm section, running at
    // a constant rate, used only for percussion input
    **noise_lfsr <<= 1;
    **noise_lfsr |= opl_emu_bitfield(**noise_lfsr, 23, 1)
        .wrapping_add(opl_emu_bitfield(**noise_lfsr, 9, 1))
        .wrapping_add(opl_emu_bitfield(**noise_lfsr, 8, 1))
        .wrapping_add(opl_emu_bitfield(**noise_lfsr, 1, 1));

    // OPL has two fixed-frequency LFOs, one for AM, one for PM

    // the AM LFO has 210*64 steps; at a nominal 50kHz output,
    // this equates to a period of 50000/(210*64) = 3.72Hz
    let mut am_counter = **lfo_am_counter;
    if am_counter >= 210 * 64 - 1 {
        **lfo_am_counter = 0;
    }

    // low 8 bits are fractional; depth 0 is divided by 2, while depth 1 is times 2
    let shift = 9 - 2 * am_depth;

    // AM value is the upper bits of the value, inverted across the midpoint
    // to produce a triangle
    let am_value = if am_counter < 105 * 64 {
        am_counter as u32
    } else {
        210 * 64 + 63 - am_counter as u32
    };
    **lfo_am = ((am_value >> shift) & 0xFF) as u8;

    // the PM LFO has 8192 steps, or a nominal period of 6.1Hz
    let pm_counter = **lfo_pm_counter;

    // PM LFO is broken into 8 chunks, each lasting 1024 steps; the PM value
    // depends on the upper bits of FNUM, so this value is a fraction and
    // sign to apply to that value, as a 1.3 value
    let pm_scale: [i32; 8] = [8, 4, 0, -4, -8, -4, 0, 4];
    return pm_scale[((pm_counter >> 10) & 7) as usize] >> (pm_depth ^ 1);
}

