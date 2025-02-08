
use std::boxed::Box;

#[derive(Clone)]
pub struct OplEmuRegisters {
    // internal state
    pub m_lfo_am_counter: u16,            // LFO AM counter
    pub m_lfo_pm_counter: u16,            // LFO PM counter
    pub m_noise_lfsr: u32,                // noise LFSR state
    pub m_lfo_am: u8,                     // current LFO AM value
    pub m_regdata: Box<[u8; 0x200]>,         // register data
    pub m_waveform: Box<[[u16; 0x400]; 8]>, // waveforms
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
    opl_emu_bitfield(regs.m_regdata[(offset + extra_offset) as usize] as u32, start, count)
}

pub fn opl_emu_registers_rhythm_enable(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0xbd, 5, 1, 0)
}
