

use std::num::Wrapping;

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let shift = start as u32;
    let mask = ((1u32 << length as u32) - 1) as u32;
    (Wrapping(value).0.wrapping_shr(shift) & mask)
}

fn opl_emu_attenuation_increment(rate: u32, index: u32) -> u32 {
    let s_increment_table: [u32; 64] = [
        0x00000000, 0x00000000, 0x10101010, 0x10101010,  // 0-3    (0x00-0x03)
        0x10101010, 0x10101010, 0x11101110, 0x11101110,  // 4-7    (0x04-0x07)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 8-11   (0x08-0x0B)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 12-15  (0x0C-0x0F)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 16-19  (0x10-0x13)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 20-23  (0x14-0x17)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 24-27  (0x18-0x1B)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 28-31  (0x1C-0x1F)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 32-35  (0x20-0x23)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 36-39  (0x24-0x27)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 40-43  (0x28-0x2B)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 44-47  (0x2C-0x2F)
        0x11111111, 0x21112111, 0x21212121, 0x22212221,  // 48-51  (0x30-0x33)
        0x22222222, 0x42224222, 0x42424242, 0x44424442,  // 52-55  (0x34-0x37)
        0x44444444, 0x84448444, 0x84848484, 0x88848884,  // 56-59  (0x38-0x3B)
        0x88888888, 0x88888888, 0x88888888, 0x88888888   // 60-63  (0x3C-0x3F)
    ];

    opl_emu_bitfield(s_increment_table[rate as usize], (4 * index) as i32, 4)
}

