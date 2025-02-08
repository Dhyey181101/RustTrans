

use std::cmp;

fn opl_emu_opl_key_scale_atten(block: u32, fnum_4msb: u32) -> u32 {
    let fnum_to_atten: [u8; 16] = [0, 24, 32, 37, 40, 43, 45, 47, 48, 50, 51, 52, 53, 54, 55, 56];
    let result = fnum_to_atten[fnum_4msb as usize]
        .wrapping_sub(((8 * (block ^ 7)) & 0xff) as u8)
        .wrapping_sub(128);
    cmp::max(0, result as i32) as u32
}

