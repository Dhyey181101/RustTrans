
use std::boxed::Box;

#[derive(Clone)]
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
    regs: &OplEmuRegisters,
    offset: u32,
    start: i32,
    count: i32,
    extra_offset: u32,
) -> u32 {
    opl_emu_bitfield(regs.m_regdata[offset as usize + extra_offset as usize] as u32, start, count)
}

pub fn opl_emu_registers_lfo_am_depth(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0xbd, 7, 1, 0)
}
