
use std::mem;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,            // LFO AM counter
    m_lfo_pm_counter: u16,            // LFO PM counter
    m_noise_lfsr: u32,                // noise LFSR state
    m_lfo_am: u8,                     // current LFO AM value
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],         // register data
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS], // waveforms
}

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &mut OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    let index = (offset + extra_offset) as usize;
    let value = regs.m_regdata[index];
    opl_emu_bitfield(value as u32, start, count)
}

fn opl_emu_registers_op_decay_rate(regs: &mut OplEmuRegisters, opoffs: u32) -> u32 {
    opl_emu_registers_byte(regs, 0x60, 0, 4, opoffs)
}
