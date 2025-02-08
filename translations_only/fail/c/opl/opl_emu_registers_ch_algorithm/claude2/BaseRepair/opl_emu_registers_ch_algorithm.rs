

use std::convert::TryInto;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct OplEmuRegisters {
    // internal state
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start.wrapping_abs() as u32) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(
    regs: &OplEmuRegisters,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    opl_emu_bitfield(
        regs.m_regdata
            .get(offset.wrapping_add(extra_offset) as usize)
            .copied()
            .unwrap_or(0) as u32,
        start as i32,
        count.try_into().unwrap(),
    )
}

fn opl_emu_registers_ch_algorithm(regs: &OplEmuRegisters, choffs: u32) -> u32 {
    opl_emu_registers_byte(regs, 0xc0, 0, 1, choffs.try_into().unwrap())
        | ((8 | (opl_emu_registers_byte(regs, 0xc3, 0, 1, choffs) << 1)))
}

