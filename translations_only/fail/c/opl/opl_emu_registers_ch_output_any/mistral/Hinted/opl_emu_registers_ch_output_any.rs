
use std::fmt;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u32,
    m_lfo_pm_counter: u32,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

impl fmt::Debug for OplEmuRegisters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement custom formatting as needed
        write!(f, "OplEmuRegisters {{ ... }}")
    }
}

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    opl_emu_bitfield(regs.m_regdata[offset as usize + extra_offset as usize] as u32, start, count)
}

fn opl_emu_registers_newflag(regs: &OplEmuRegisters) -> bool {
    opl_emu_registers_byte(regs, 0x105, 0, 1, 0) != 0
}

fn opl_emu_registers_ch_output_any(regs: &OplEmuRegisters, choffs: u32) -> bool {
    if opl_emu_registers_newflag(regs) {
        opl_emu_registers_byte(regs, 0xc0 + choffs, 4, 4, 0) != 0
    } else {
        true
    }
}
