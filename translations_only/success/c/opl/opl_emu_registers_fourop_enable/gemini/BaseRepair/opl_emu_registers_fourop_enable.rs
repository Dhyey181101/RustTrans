
use std::boxed::Box;

#[derive(Clone, Copy)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; 0x200],
    m_waveform: [[u16; 0x400]; 8],
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(
    regs: &Box<OplEmuRegisters>,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    opl_emu_bitfield(
        regs.m_regdata[(offset + extra_offset) as usize] as u32,
        start as i32,
        count as i32,
    )
}

fn opl_emu_registers_fourop_enable(regs: &Box<OplEmuRegisters>) -> u32 {
    opl_emu_registers_byte(regs, 0x104, 0, 6, 0)
}
