
use std::boxed::Box;

#[derive(Clone)]
pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: Box<[u8]>,
    pub m_waveform: Box<[Box<[u16]>]>,
}

impl OplEmuRegisters {
    pub fn opl_emu_bitfield(&self, value: u32, start: i32, length: i32) -> u32 {
        ((value >> start) & ((1 << length) - 1)) as u32
    }

    pub fn opl_emu_registers_byte(
        &self,
        offset: u32,
        start: u32,
        count: u32,
        extra_offset: u32,
    ) -> u32 {
        self.opl_emu_bitfield(
            self.m_regdata[offset as usize + extra_offset as usize] as u32,
            start as i32,
            count as i32,
        )
    }

    pub fn opl_emu_registers_newflag(&self) -> u32 {
        self.opl_emu_registers_byte(0x105, 0, 1, 0)
    }

    pub fn opl_emu_registers_ch_output_3(&self, choffs: u32) -> u32 {
        if self.opl_emu_registers_newflag() != 0 {
            self.opl_emu_registers_byte(0xc0 + choffs, 7, 1, 0)
        } else {
            0
        }
    }
}
