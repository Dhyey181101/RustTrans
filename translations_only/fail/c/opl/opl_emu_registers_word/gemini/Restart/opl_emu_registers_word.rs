
use std::boxed::Box;

#[derive(Debug)]
pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: Box<[u8; 0x200]>,
    pub m_waveform: Box<[[u16; 0x400]; 8]>,
}

pub fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

pub fn opl_emu_registers_byte(
    regs: &mut OplEmuRegisters,
    offset: u32,
    start: i32,
    count: i32,
    extra_offset: u32,
) -> u32 {
    opl_emu_bitfield(regs.m_regdata[offset as usize + extra_offset as usize] as u32, start, count)
}

pub fn opl_emu_registers_word(
    regs: &mut OplEmuRegisters,
    offset1: u32,
    start1: i32,
    count1: i32,
    offset2: u32,
    start2: i32,
    count2: i32,
    extra_offset: u32,
) -> u32 {
    (opl_emu_registers_byte(regs, offset1, start1, count1, extra_offset) << count2)
        | opl_emu_registers_byte(regs, offset2, start2, count2, extra_offset)
}
