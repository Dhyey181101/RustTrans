
use std::boxed::Box;

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_attenuation_increment(rate: u32, index: u32) -> u32 {
    let s_increment_table: Box<[u32]> = Box::new([
        0x00000000, 0x00000000, 0x10101010, 0x10101010,  
        0x10101010, 0x10101010, 0x11101110, 0x11101110,  
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  
        0x10101010, 0x10111010, 0x11101110, 0x11111110,
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  
        0x10101010, 0x10111010, 0x11101110, 0x11111110,
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  
        0x10101010, 0x10111010, 0x11101110, 0x11111110,
        0x11111111, 0x21112111, 0x21212121, 0x22212221,  
        0x22222222, 0x42224222, 0x42424242, 0x44424442,  
        0x44444444, 0x84448444, 0x84848484, 0x88848884,  
        0x88888888, 0x88888888, 0x88888888, 0x88888888
    ]);

    opl_emu_bitfield(*s_increment_table.get(rate as usize).unwrap(), 4*index as i32, 4)
}
