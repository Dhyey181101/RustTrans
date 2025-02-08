
use std::boxed::Box;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct OplEmuRegisters {
    // internal state
    m_lfo_am_counter: u16, // LFO AM counter
    m_lfo_pm_counter: u16, // LFO PM counter
    m_noise_lfsr: u32,     // noise LFSR state
    m_lfo_am: u8,          // current LFO AM value
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS]>, // register data
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>, // waveforms
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    opl_emu_bitfield(regs.m_regdata[offset.wrapping_add(extra_offset as u32) as usize] as u32, start as i32, count as i32)
}

fn opl_emu_registers_newflag(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x105, 0, 1, 0)
}

fn opl_emu_registers_ch_output_1(regs: &OplEmuRegisters, choffs: u32) -> u32 {
    if opl_emu_registers_newflag(regs) > 0 {
        opl_emu_registers_byte(regs, 0xc0_u32.wrapping_add(choffs), 5, 1, 0) 
    } else {
        1
    }
}

