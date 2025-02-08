

use std::mem;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

impl OplEmuRegisters {
    fn opl_emu_bitfield(&self, value: u32, start: i32, length: i32) -> u32 {
        (value >> start) & ((1 << length) - 1)
    }

    fn opl_emu_registers_byte(&self, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
        self.opl_emu_bitfield(
            (self.m_regdata[offset as usize + extra_offset as usize] as u32).wrapping_add(1 << (8 * extra_offset as u32))
                .wrapping_sub(1 << (8 * (extra_offset + count) as u32)),
            start as i32,
            count as i32,
        )
    }

    fn opl_emu_registers_op_eg_sustain(&self, opoffs: u32) -> u32 {
        self.opl_emu_registers_byte(0x20, 5, 1, opoffs)
    }
}

fn main() {
    // Test cases go here
}

